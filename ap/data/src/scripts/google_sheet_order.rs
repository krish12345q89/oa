use chrono::{ Local };
use jsonwebtoken::{ Algorithm, EncodingKey, Header };
use reqwest::Client;
use serde::{ Deserialize, Serialize };
use serde_json::{ Value };
use std::error::Error;
use std::time::{ SystemTime, UNIX_EPOCH };

use crate::lmdb::order::DBOrder;
use crate::lmdb::utils::{ init_db, DB };
use crate::schema::order::Order;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}

pub async fn generate_token(
    client_email: &str,
    private_key: &str
) -> Result<String, Box<dyn Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;

    let claims = Claims {
        iss: client_email.to_string(),
        scope: "https://www.googleapis.com/auth/spreadsheets".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: now + 3600,
        iat: now,
    };

    let jwt = jsonwebtoken::encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_bytes())?
    )?;

    let response = Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(
            &[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ]
        )
        .send().await?
        .json::<Value>().await?;

    response["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Failed to get access token".into())
}

async fn fetch_sheet_data(
    access_token: &str,
    spreadsheet_id: &str,
    sheet_name: &str
) -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
        spreadsheet_id,
        sheet_name
    );

    let response = Client::new().get(&url).bearer_auth(access_token).send().await?;

    if !response.status().is_success() {
        return Err(format!("API request failed: {}", response.text().await?).into());
    }

    Ok(response.json::<Value>().await?)
}

async fn process_sheet1_data(
    access_token: &str,
    spreadsheet_id: &str,
    db: &DB
) -> Result<(), Box<dyn Error>> {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let timestamp = now.to_rfc3339();

    // Fetch columns B (Marketplace) and F (Order ID) from Sheet1
    let columns = ["B", "F"];
    let mut results = Vec::new();

    for col in &columns {
        let range = format!("Sheet1!{}:{}", col, col);
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
            spreadsheet_id,
            range
        );

        let response = Client::new().get(&url).bearer_auth(access_token).send().await?;

        if !response.status().is_success() {
            return Err(
                format!("Failed to fetch column {}: {}", col, response.text().await?).into()
            );
        }

        let text = response.text().await?;
        let data: Value = serde_json::from_str(&text)?;
        let values = data["values"].as_array().ok_or("Missing values array")?.clone(); // Clone the array to own the data
        results.push(values);
    }

    let marketplace_values = &results[0];
    let order_id_values = &results[1];

    // Process data starting from row 1 (skip header if exists)
    let start_row = if marketplace_values.len() > 1 && order_id_values.len() > 1 { 1 } else { 0 };

    for i in start_row..marketplace_values.len().min(order_id_values.len()) {
        let marketplace = marketplace_values[i][0].as_str().unwrap_or("").to_string();
        let order_id = order_id_values[i][0].as_str().unwrap_or("").to_string();
        let s = order_id.clone();
        let order = Order {
            id: order_id.clone(),
            marketplace,
            order_id,
            return_order: None,
            shopify_id: None,
            market_place_code: None,
            returned_sku: None,
            offer_sku: None,
            matched_sku: None,
            match_type: None,
            row_number: Some(i as u32),
            manual_confirmation: None,
            status: None,
            qty: None,
            main_updated: Some("SYNCED".to_string()),
            date: date_str.clone(),
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        db
            .get_single(s)
            .map_err(|e| format!("Failed to get order: {}", e))?
            .map(|existing_order| {
                if existing_order != order {
                    db.put(order.clone()).map_err(|e| format!("Failed to update order: {}", e))
                } else {
                    Ok(())
                }
            })
            .unwrap_or_else(|| {
                db.insert(order).map_err(|e| format!("Failed to insert order: {}", e))
            })?;
    }
    let res = db.get().map_err(|e| format!("Failed to retrieve orders: {}", e))?;
    println!("Retrieved Orders: {:#?}", res);
    Ok(())
}

pub async fn generate_access_token() -> Result<(), Box<dyn Error>> {
    let db = init_db("./data/lmdb").await?;
    let private_key =
        "-----BEGIN PRIVATE KEY-----\nMIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQC73FeydFYHs15D\n/57HyQ70AIJhbmokT69v8GXhvCAExgei41AOBPjAMbCwVs7TGdcHsTFLsXbNbgbT\nJy9DrxYUQHXKWT7But/ZlUTek7z2LqW5TK3qNmsfgBimY5l7m50IyhOEzrLGEV9j\nqIOzwE/XVD0UEHglo5wasTrTwtp3aFV7KzcfbQm5Blr09DuhuxsYkvHkEvsYttwa\n0m/2Dj05/V+VuBPJTuTMnA5t5F8rqn+hzeCx0TN8BmNmoFF3C6kLQK75XcSfJZoc\nenGVc/bF7BF+FmBHWGPNZONu9yVuf6Gdwwptk5znYbtR1k6ZOfOqgrrD2k40d8ug\nISDhDRL3AgMBAAECggEAAQprcuSxn6q1iVTIAzlwvWnGIfSDz6mRWOJYhLsl1RWk\ns8ZOIRxXWhVz6onusI/8yiTxYp1+AVihC9+WSVNZkdhT7NcLnBLnEZ6vA7FZmAwD\nDKoYUUG5kQU5ljREsd8ThnP/qRg3Vrtxb14Ej5XkNbErbAo1q3oQFXEQupk8zQ/T\nxMmteUX5c1m2D/+P56GjbxnsHOoUhXpyhkhbXQTBy/hj1ADOsBM85Hl6IdN3ZIuO\nqSixsDe9pj/NojKVzjK52ioORLKsNB1yO8mIm3gOJfrlO8c2uOTazxdQvu0B+tf2\nNMFg4RBQkkfGx0FGcHGV90NCVsrQkBCfHGjr3FX0QQKBgQDcLumU7VbREAUSN0vo\nAD5+6cmAr6QHwjR0MOuJI6aEJrYel+Jk7ji0yCjPFvxU1WQdEr0Ew8lN7eKzH5Qm\nM5xBtkio3hDubvMu10k/A+enqKbJcOpsgNVuA/wkwwBdYq32EESJN33wFLCQ9iCF\no8fUpux2hvea5L+VDtJOF6DmSQKBgQDaa2xWSN+0/QyQs7f9mBpi63dfcW9ubE6j\nylaPaaF3MZddari9/roWq4KqA3zVElQNRgW58Fdg6rBMJl/zoPc/txY/EC8qqXnD\nfphykRS1X4//fQMIxSaXa/8sv8xVV0Q84edeeRmqVFrPIHE7Dfue/xK7PhAd88y0\nGSp20MUvPwKBgBXLUvWZ1IkXE9lsvce2FnmLfJWPTSYzc+u4V+gYLkhQaKB4mkEx\nT02drmRpOwrOoH08OJd7JbbgABuI2ao6W5Ipj+GfMX/YXZvaVXa2VTzENdYkph+d\nVQLxxAiDgMq9lMiRbadDZeTYt21x32A7CPGkoC1PSLz9PXVspSZzskp5AoGAMQqr\nsZf9OssSli70IemUCx/plrGGnpmM8rPMybii+3tUDDKZNnfKWqq51Oihj8nku3I8\neOphBC7N1NtM8gvzWAgI47IDlSWmZGG5Ywf2SV8imu/7zW1O4/Lowahy/bpxZOYo\nKAsy2w7DsPwM0ICsPZ/yGb6uqbsC/HEmGrV3gMsCgYAftsY+oxGxHSF8Us4KGPta\nWjAg3PATgr1phejWaHXXjSgaDb1iGT5Q74dTg9vRY7bkvwp3dZR2PPK/xn9BM9VQ\nXM5k/QnfQPz+fPCePQ9dA/W4e7SCM/a80rbYUYKZvgtsFVRnFhruev5sXgO62PL2\nQor4cdJ5794vuAlF6uf+jQ==\n-----END PRIVATE KEY-----\n";
    let client_email = "krish-323@genuine-plating-453110-q0.iam.gserviceaccount.com";
    let access_token = generate_token(client_email, private_key).await?;
    let spreadsheet_id = "16pzLDZosE9HIhrWRrxc8ZkWERhWf0LVnqx0SI4e_eas";

    // Process Sheet1 data (Order IDs and Marketplaces)
    process_sheet1_data(&access_token, spreadsheet_id, &db).await?;

    // Fetch and process Sheet2 data if needed
    let sheet2_data = fetch_sheet_data(&access_token, spreadsheet_id, "Sheet2").await?;
    println!("Sheet2 Data:\n{}", serde_json::to_string_pretty(&sheet2_data)?);

    Ok(())
}

