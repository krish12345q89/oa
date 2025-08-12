use std::collections::HashMap;
use crate::schema::order::Order;

pub fn get_sheet1_column_map() -> HashMap<&'static str, usize> {
    let mut map = HashMap::new();
    map.insert("MARTKETPLACE", 1);
    map.insert("BIN_RACK", 4);
    map.insert("ORDER_ID", 5);
    map.insert("RETURN_REAS", 6);
    map.insert("REFUND_YES", 7);
    map.insert("DATE", 8);
    map.insert("REFUNDED", 9);
    map.insert("STOCK_ADDED", 10);
    map.insert("REFUND_DATE", 11);
    map.insert("MATCH_TYPE", 12);
    map.insert("FRASER_CLASSIFICATION", 13);
    map
}

pub fn get_sheet2_column_map() -> HashMap<&'static str, usize> {
    let mut map = HashMap::new();
    map.insert("RETURN_ORDER", 0);
    map.insert("SHOPIFY_ID", 1);
    map.insert("MAKRETPLACE", 2);
    map.insert("RETURNED_SKU", 3);
    map.insert("OFFER_SKU", 4);
    map.insert("MATCHED_SKU", 5);
    map.insert("MATCH_TYPE", 6);
    map.insert("ROW_NUMBER", 7);
    map.insert("MANUAL_CONFIRMAT", 8);
    map.insert("STATUS", 9);
    map.insert("MARKETPLACE", 10);
    map.insert("QTY", 11);
    map.insert("MAINUPDATED", 12);
    map
}


pub fn update_rows_for_order(
    order: &Order,
    existing_row_sheet1: &mut Vec<String>,
    existing_row_sheet2: &mut Vec<String>,
) {
    let sheet1_map = get_sheet1_column_map();
    let sheet2_map = get_sheet2_column_map();

    let mut update_cell = |row: &mut Vec<String>, map: &HashMap<&str, usize>, key: &str, value: &str| {
        if let Some(&idx) = map.get(key) {
            if idx >= row.len() {
                row.resize(idx + 1, "".to_string());
            }
            row[idx] = value.to_string();
        }
    };

    
    // marketplace is mandatory String, update both sheets (note: sheet1 key typo "MARTKETPLACE")
    update_cell(existing_row_sheet1, &sheet1_map, "MARTKETPLACE", &order.marketplace);
    update_cell(existing_row_sheet2, &sheet2_map, "MARKETPLACE", &order.marketplace);

    if let Some(return_order) = order.return_order {
        let val_str = return_order.to_string();
        update_cell(existing_row_sheet1, &sheet1_map, "RETURN_REAS", &val_str);
        update_cell(existing_row_sheet2, &sheet2_map, "RETURN_ORDER", &val_str);
    }

    if let Some(ref manual_confirmation) = order.manual_confirmation {
        update_cell(existing_row_sheet1, &sheet1_map, "REFUND_YES", manual_confirmation);
        update_cell(existing_row_sheet2, &sheet2_map, "MANUAL_CONFIRMAT", manual_confirmation);
    }

    if let Some(ref status) = order.status {
        update_cell(existing_row_sheet1, &sheet1_map, "BIN_RACK", status);
        update_cell(existing_row_sheet2, &sheet2_map, "STATUS", status);
    }

    if let Some(qty) = order.qty {
        let val_str = qty.to_string();
        update_cell(existing_row_sheet1, &sheet1_map, "STOCK_ADDED", &val_str);
        update_cell(existing_row_sheet2, &sheet2_map, "QTY", &val_str);
    }

    if let Some(ref main_updated) = order.main_updated {
        update_cell(existing_row_sheet1, &sheet1_map, "REFUND_DATE", main_updated);
        update_cell(existing_row_sheet2, &sheet2_map, "MAINUPDATED", main_updated);
    }

    // Date, created_at, updated_at update only sheet1 because sheet2 doesn't have those columns
    update_cell(existing_row_sheet1, &sheet1_map, "DATE", &order.date);
    update_cell(existing_row_sheet1, &sheet1_map, "REFUNDED", &order.created_at);
    update_cell(existing_row_sheet1, &sheet1_map, "MATCH_TYPE", &order.updated_at);
}