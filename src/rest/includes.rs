use core::fmt;
use std::{collections::HashMap, path::Display};

use bitflags::bitflags;

pub enum IdentityInclude {
    Memberships(MembershipInclude),
    Campaign,
}

impl fmt::Display for IdentityInclude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IdentityInclude::Memberships(_) => write!(f, "memberships"),
            IdentityInclude::Campaign => write!(f, "campaign"),
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MembershipInclude: u16 {
        const CAMPAIGN_LIFETIME_SUPPORT_CENTS = 1 << 0;
        const CURRENTLY_ENTITLED_AMOUNT_CENTS = 1 << 1;
        const EMAIL = 1 << 2;
        const FULL_NAME = 1 << 3;
        const IS_FOLLOWER = 1 << 4;
        const LAST_CHARGE_DATE = 1 << 5;
        const LAST_CHARGE_STATUS = 1 << 6;
        const LIFETIME_SUPPORT_CENTS = 1 << 7;
        const NEXT_CHARGE_DATE = 1 << 8;
        const NOTE = 1 << 9;
        const PATRON_STATUS = 1 << 10;
        const PLEDGE_CADENCE = 1 << 11;
        const PLEDGE_RELATIONSHIP_START = 1 << 12;
        const WILL_PAY_AMOUNT_CENTS = 1 << 13;
    }
}

impl MembershipInclude {
    pub const fn flags_as_str(&self) -> [(Self, &str); 14] {
        [
            (
                MembershipInclude::CAMPAIGN_LIFETIME_SUPPORT_CENTS,
                "campaign_lifetime_support_cents",
            ),
            (
                MembershipInclude::CURRENTLY_ENTITLED_AMOUNT_CENTS,
                "currently_entitled_amount_cents",
            ),
            (MembershipInclude::EMAIL, "email"),
            (MembershipInclude::FULL_NAME, "full_name"),
            (MembershipInclude::IS_FOLLOWER, "is_follower"),
            (MembershipInclude::LAST_CHARGE_DATE, "last_charge_date"),
            (MembershipInclude::LAST_CHARGE_STATUS, "last_charge_status"),
            (
                MembershipInclude::LIFETIME_SUPPORT_CENTS,
                "lifetime_support_cents",
            ),
            (MembershipInclude::NEXT_CHARGE_DATE, "next_charge_date"),
            (MembershipInclude::NOTE, "note"),
            (MembershipInclude::PATRON_STATUS, "patron_status"),
            (MembershipInclude::PLEDGE_CADENCE, "pledge_cadence"),
            (
                MembershipInclude::PLEDGE_RELATIONSHIP_START,
                "pledge_relationship_start",
            ),
            (
                MembershipInclude::WILL_PAY_AMOUNT_CENTS,
                "will_pay_amount_cents",
            ),
        ]
    }

    pub fn as_query(&self) -> String {
        self.flags_as_str()
            .into_iter()
            .filter(|(flag, _)| self.contains(*flag))
            .map(|(_, value)| value)
            .collect::<Vec<&str>>()
            .join(",")
    }
}
