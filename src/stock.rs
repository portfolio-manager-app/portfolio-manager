// This file will be responsible to retrieve the stock data from the Yahoo Finance API
// and return it to the main.rs file
// It uses rust curl to make the request to the API
use curl::easy::Easy;

// Function to get the stock data from the Yahoo Finance API
// Returns the stock data
pub fn get_stock_data(stock: &str) -> () {
    let mut easy = Easy::new();
    easy.url(&["https://query1.finance.yahoo.com/v7/finance/quote?symbols=", stock].join("")).unwrap();
    easy.perform().unwrap();
}

// Test for get_stock_data function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stock_data() {
        get_stock_data("AAPL");
        get_stock_data("MSFT");
        get_stock_data("UNEXISTING_TIKR");
    }
} 
