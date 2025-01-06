use bitflags::bitflags;

bitflags! {
    pub struct IdentityInclude: u8 {
        const MEMBERSHIPS = 1 << 0;
        const CAMPAIGN = 1 << 1;
    }
}

impl IdentityInclude {
    pub(crate) fn as_query(&self) -> String {
        let mut included = Vec::new();

        if self.contains(IdentityInclude::MEMBERSHIPS) {
            included.push("memberships");
        }
        if self.contains(IdentityInclude::CAMPAIGN) {
            included.push("campaign");
        }

        included.join(",")
    }
}

bitflags! {
    pub struct CampaignInclude: u8 {
        const TIERS = 1 << 0;
        const CREATOR = 1 << 1;
        const BENEFITS = 1 << 2;
        const GOALS = 1 << 3;
    }
}

impl CampaignInclude {
    pub(crate) fn as_query(&self) -> String {
        let mut included = Vec::new();

        if self.contains(CampaignInclude::TIERS) {
            included.push("tiers");
        }
        if self.contains(CampaignInclude::CREATOR) {
            included.push("creator");
        }
        if self.contains(CampaignInclude::BENEFITS) {
            included.push("benefits");
        }
        if self.contains(CampaignInclude::GOALS) {
            included.push("goals");
        }

        included.join(",")
    }
}

bitflags! {
    pub struct MemberInclude: u8 {
        const ADDRESS = 1 << 0;
        const CAMPAIGN = 1 << 1;
        const CURRENTLY_ENTITLED_TIERS = 1 << 2;
        const USER = 1 << 3;
        const PLEDGE_HISTORY = 1 << 4;
    }
}

impl MemberInclude {
    pub(crate) fn as_query(&self) -> String {
        let mut included = Vec::new();

        if self.contains(MemberInclude::ADDRESS) {
            included.push("address");
        }
        if self.contains(MemberInclude::CAMPAIGN) {
            included.push("campaign");
        }
        if self.contains(MemberInclude::CURRENTLY_ENTITLED_TIERS) {
            included.push("currently_entitled_tiers");
        }
        if self.contains(MemberInclude::USER) {
            included.push("user");
        }
        if self.contains(MemberInclude::PLEDGE_HISTORY) {
            included.push("pledge_history");
        }

        included.join(",")
    }
}

bitflags! {
    pub struct PostInclude: u8 {
        const USER = 1 << 0;
        const CAMPAIGN = 1 << 1;
    }
}

impl PostInclude {
    pub(crate) fn as_query(&self) -> String {
        let mut included = Vec::new();

        if self.contains(PostInclude::USER) {
            included.push("user");
        }
        if self.contains(PostInclude::CAMPAIGN) {
            included.push("campaign");
        }

        included.join(",")
    }
}
