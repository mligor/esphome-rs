use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EspHomeError {
	#[error("The password was not valid")]
	InvalidPassword,

	#[error("Received an unexpected response type (expected {expected:?}, received {received:?})")]
	UnexpectedResponse {
		expected: MessageType,
		received: u32,
	},

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Protobuf error: {0}")]
	Protobuf(#[from] protobuf::ProtobufError),

	#[error("System time error: {0}")]
	SystemTime(#[from] std::time::SystemTimeError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
	Binary(bool),
	Measurement(f32),
	Text(String),
	LightState((bool, f32)), //TODO: add all states for light
	FanState(bool), //TODO: add all states for fan
	LockState(crate::api::LockState), 
}

#[derive(Debug)]
pub struct ExtendedInfo {
	pub(crate) object_id: String,
	pub(crate) unique_id: String,
}

impl ExtendedInfo {
    
	pub fn object_id(&self) -> String {
		self.object_id.clone()
	}

	pub fn unique_id(&self) -> String {
		self.unique_id.clone()
	}
}

#[derive(Debug)]
pub struct EntityInfo {
	pub(crate) name: String,
	pub(crate) key: u32,
}

#[derive(Debug)]
pub struct Entity {
	info: EntityInfo,
	kind: EntityKind,
}

impl Entity {
	pub(crate) fn new(info: EntityInfo, kind: EntityKind) -> Entity {
		Entity { info, kind }
	}

	pub fn key(&self) -> u32 {
		self.info.key
	}

	pub fn name(&self) -> String {
		self.info.name.clone()
	}

	pub fn kind(&self) -> &EntityKind {
		&self.kind
	}

	pub fn extended_info(&self) -> ::std::option::Option<&ExtendedInfo> {
		match &self.kind {
			EntityKind::BinarySensor(ei) => Some(ei),
			EntityKind::Camera(ei) => Some(ei),
			EntityKind::Climate(ei) => Some(ei),
			EntityKind::Cover(ei) => Some(ei),
			EntityKind::Fan(ei) => Some(ei),
			EntityKind::Light(ei) => Some(ei),
			EntityKind::Number(ei) => Some(ei),
			EntityKind::Select(ei) => Some(ei),
			EntityKind::Sensor(ei) => Some(ei),
			EntityKind::Services => None,
			EntityKind::Switch(ei) => Some(ei),
			EntityKind::TextSensor(ei) => Some(ei),
			EntityKind::Lock(ei) => Some(ei),
			EntityKind::Button(ei) => Some(ei),
		}.clone()
	}
 }

#[derive(Debug)]
pub enum EntityKind {
	BinarySensor(ExtendedInfo),
	Camera(ExtendedInfo),
	Climate(ExtendedInfo),
	Cover(ExtendedInfo),
	Fan(ExtendedInfo),
	Light(ExtendedInfo),
	Number(ExtendedInfo),
	Select(ExtendedInfo),
	Sensor(ExtendedInfo),
	Services,
	Switch(ExtendedInfo),
	TextSensor(ExtendedInfo),
	Lock(ExtendedInfo),
	Button(ExtendedInfo),
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum MessageType {
	HelloRequest = 1,
	HelloResponse = 2,
	ConnectRequest = 3,
	ConnectResponse = 4,
	DisconnectRequest = 5,
	DisconnectResponse = 6,
	PingRequest = 7,
	PingResponse = 8,
	DeviceInfoRequest = 9,
	DeviceInfoResponse = 10,
	ListEntitiesRequest = 11,
	ListEntitiesBinarySensorResponse = 12,
	ListEntitiesCoverResponse = 13,
	ListEntitiesFanResponse = 14,
	ListEntitiesLightResponse = 15,
	ListEntitiesSensorResponse = 16,
	ListEntitiesSwitchResponse = 17,
	ListEntitiesTextSensorResponse = 18,
	ListEntitiesDoneResponse = 19,
	SubscribeStatesRequest = 20,

	BinarySensorStateResponse = 21,
	CoverStateResponse = 22,
	FanStateResponse = 23,
	LightStateResponse = 24,
	SensorStateResponse = 25,
	SwitchStateResponse = 26,
	TextSensorStateResponse = 27,
	LockStateResponse = 59,

	ClimateStateResponse = 47,
	NumberStateResponse = 50,
	SelectStateResponse = 53,

	GetTimeRequest = 36,
	GetTimeResponse = 37,

	ListEntitiesServicesResponse = 41,
	ListEntitiesCameraResponse = 43,
	ListEntitiesClimateResponse = 46,
	ListEntitiesNumberResponse = 49,
	ListEntitiesSelectResponse = 52,
	ListEntitiesLockResponse = 58,
	ListEntitiesButtonResponse = 61,

}
