pub enum Platform {
    Android,
    IOS,
}

pub enum DeviceKind<T> {
    SmartPhone(T),
    Tablet(T),
}

pub struct DeviceInfo {
    kind: DeviceKind<Platform>,
    model_year: u16,
}

pub trait Device<DeviceKind> {
    fn info() -> DeviceInfo;
}
