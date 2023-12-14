pub struct TimeBreakdown {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32
}

pub enum ConnectionProp {
    MicroserviceId,
    InstanceId
}

pub enum WindowIndexingType {
    FromZero,
    SeqFromStart
}
