//! A library that handles calculating, rounding, displaying, and parsing units of money according
//! to ISO 4217 standards. The main item exported by the library is `Money`.
//!
//! # Use
//!
//! ```edition2018
//! use rusty_money::money;
//! use rusty_money::Money;
//!
//! // The easiest way to create Money objects is by using the money! macro
//! // which accepts amounts strings or integers and currencies as strings:
//!
//! money!("-200.00", "USD") == money!(-200, "USD"); // true
//!
//! // Money objects can be initialized in a few other convenient ways:
//!
//! use rusty_money::Currency;
//! use rusty_money::Iso::*;
//!
//! Money::new(200000, Currency::get(USD));         // amount = 2000 USD
//! Money::from_major(2000, Currency::get(USD));    // amount = 2000 USD
//! Money::from_minor(200000, Currency::get(USD));  // amount = 2000 USD
//! Money::from_str("2,000.00", "USD").unwrap();    // amount = 2000 USD
//!
//! // Money objects support arithmetic operations:
//!
//! money!(100, "USD") + money!(100, "USD"); // amount = 200 USD
//! money!(100, "USD") - money!(100, "USD"); // amount = 0 USD
//! money!(1, "USD") * 3;                    // amount = 3 USD
//! money!(3, "USD") / 3;                    // amount = 0.333333333... USD
//!
//! // Money objects can be compared:
//!
//! let hundred = money!(100, "USD");
//! let thousand = money!(1000, "USD");
//! println!("{}", thousand > hundred);     // false
//! println!("{}", thousand.is_positive()); // true
//!
//! // Money objects format themselves when printed:
//!
//! let usd = money!("-2000.009", "USD");
//! let eur = money!("-2000.009", "EUR");
//! println!("{}", usd); // -$2,000.01
//! println!("{}", eur); // -€2.000,01;
//!
//! // Money objects don't round by default, though you can make this happen manually:
//!
//! let mut usd = money!("-2000.009", "USD");  // amount = 2000.009
//! usd.round();                               // amount = 2000.01
//!
//! // Money objects can be exchange from one currency to another by setting up an ExchangeRate:
//!
//! use rusty_money::Exchange;
//! use rusty_money::ExchangeRate;
//! use rust_decimal_macros::*;
//!
//! let rate = ExchangeRate::new(Currency::get(USD), Currency::get(EUR), dec!(1.1)).unwrap();
//! rate.convert(money!(1000, "USD")); // 1,100 EUR
//!
//! // ExchangeRate objects can be stored and retrieved from a central Exchange:
//!
//! let mut exchange = Exchange::new();
//! exchange.add_or_update_rate(&rate);
//! exchange.get_rate(Currency::get(USD), Currency::get(EUR));
//! ```

mod currency;
mod error;
mod exchange;
mod format;
mod locale;
mod money;

pub use currency::*;
pub use error::MoneyError;
pub use exchange::*;
pub use format::*;
pub use locale::*;
pub use money::*;

#[macro_use]
extern crate lazy_static;
