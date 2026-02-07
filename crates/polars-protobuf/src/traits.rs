use bytes::buf::Buf;
use prost::{DecodeError, Message};
pub trait ArrowMessage: Sized + Clone {
    type ProstMessage: Message + Default;

    fn from_prost(message: Self::ProstMessage) -> Self;

    fn to_prost(self) -> Self::ProstMessage;

    fn decode<B: Buf>(buf: B) -> Result<Self, DecodeError> {
        let prost_message = <Self::ProstMessage as Message>::decode(buf)?;
        Ok(Self::from_prost(prost_message))
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        let prost_message = self.clone().to_prost();
        <Self::ProstMessage as Message>::encode_to_vec(&prost_message)
    }
}
