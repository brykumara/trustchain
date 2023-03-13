use ssi::vc::Credential;
use thiserror::Error;
use ssi::jwk::JWK;
use amcl_wrapper::field_elem::FieldElement;
use amcl_wrapper::group_elem::GroupElement;

// pk = (X, (Y,Y'i)i in I, {Z_ij})
pub struct PublicKey{ 
    pub X: FieldElement,
    pub Y_Pair: Vec<Vec<FieldElement>>,
    pub Z_i: Vec<Vec<FieldElement>>
}
// sk = (x,y1,…,yn)
pub struct SecretKey{
    pub x: FieldElement,
    pub y: Vec<FieldElement>
}
// sig = (identity, identity, sigma1, sigma2)
pub struct PSSignature { 
    pub sigma_1: SignatureGroup,
    pub sigma_2: SignatureGroup,
    pub sigma_prime_1: SignatureGroup,
    pub sigma__prime2: SignatureGroup,
}

pub struct Param {
    pub g: SignatureGroup,
    pub g_tilde: VerkeyGroup,
}


pub fn GenerateParam(label: &[u8])->Self{
    let g = SignatureGroup::from_msg_hash(&[label, " : g".as_bytes()].concat());
    let g_tilde = VerkeyGroup::from_msg_hash(&[label, " : g_tilde".as_bytes()].concat());
    Self {g, g_tilde}
}






fn main(){
    let params = GenerateParam("test".as_bytes());
    println(params);
}