use crate::currency::Currency;
use crate::locale::Locale;
use std::fmt;

// Allows iterating over the Iso Enum
macro_rules! define_enum {
    ($Name:ident { $($Variant:ident),* $(,)* }) =>
    {
        #[derive(Debug)]
        pub enum $Name {
            $($Variant),*,
        }
        pub const ISO_CURRENCIES: &'static [$Name] = &[$($Name::$Variant),*];
    }
}

// Enum that represents every ISO Currency
define_enum!(Iso {
    AED,
    BHD,
    EUR,
    GBP,
    INR,
    USD,
});

impl fmt::Display for Iso {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Returns Currency given an Iso Enum.
pub fn from_enum(code: &Iso) -> Currency {
    use Iso::*;
    use Locale::*;

    match code {
        AED => Currency {
            default_locale: EnUs,
            exponent: 2,
            iso_alpha_code: "AED",
            iso_numeric_code: "784",
            name: "United Arab Emirates Dirham",
            symbol: "د.إ",
            symbol_first: false,
        },
        BHD => Currency {
            default_locale: EnUs,
            exponent: 3,
            iso_alpha_code: "BHD",
            iso_numeric_code: "048",
            name: "Bahraini Dinar",
            symbol: "ب.د",
            symbol_first: true,
        },
        EUR => Currency {
            default_locale: EnEu,
            exponent: 2,
            iso_alpha_code: "EUR",
            iso_numeric_code: "978",
            name: "Euro",
            symbol: "€",
            symbol_first: true,
        },
        GBP => Currency {
            default_locale: EnUs,
            exponent: 2,
            iso_alpha_code: "GBP",
            iso_numeric_code: "826",
            name: "British Pound",
            symbol: "£",
            symbol_first: true,
        },
        INR => Currency {
            default_locale: EnIn,
            exponent: 2,
            iso_alpha_code: "INR",
            iso_numeric_code: "356",
            name: "Indian Rupee",
            symbol: "₹",
            symbol_first: true,
        },
        USD => Currency {
            default_locale: EnUs,
            exponent: 2,
            iso_alpha_code: "USD",
            iso_numeric_code: "840",
            name: "United States Dollar",
            symbol: "$",
            symbol_first: true,
        },
    }
}
