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
}

pub fn KeyGen(secparam : u128, n : u128) -> (PublicKey , SecretKey) {
    // Generate (g,g_tilde) as random elements from Group 1 and Group 2
    let g = SignatureGroup::from_msg_hash(&[label, " : g".as_bytes()].concat());
    let g_tilde = VerkeyGroup::from_msg_hash(&[label, " : g_tilde".as_bytes()].concat());
    // Generate n+1 numbers x,y1,...,yn from Z_p
    let x = FieldElement::random();
    let mut y = Vec::new();
    for i in 0..n{
        y.push(FieldElement::random());
    }
    
    // X = g^x
    let X = &g * &x;
    // Y_i = g^(y_i)
    let mut Y = Vec::new();
    for i in 0..n{
        Y.push(&g * &y_i);
    } 
    // Y_tilde_i = g_tilde^(y_i)
    let mut Y_tilde = Vec::new();
    for i in 0..n{
        Y_tilde.push(&g_tilde * &y_i);
    }
    // Create Y' = (Y_i,Y_tilde_i) indexed by i
    let mut Y_Pair = Vec::new();
    for i in 0..n{
        Y_Pair.push((Y[i],Y_tilde[i]))
    }
    
    // Z_i,j = g^(y_i * y_j) for 1 =< j, i =< n where i =/= j
    let mut Z_i = Vec::new();
    for i in 0..n{
        let j = 0;
        let mut Z_i_j = Vec::new();
        if (j != i, j <= n) {
            Z_i_j.push(&g * (&y_i * y_j));
            j += 1;
            Z_i.push(Z_i_j);
        } else {
            // increment j anyway
            j += 1;
        }
    }
    
    (PublicKey {X, Y_Pair, Z_i} , SecretKey {x,y})
}


// Sign(sk, [m_i] i=1...n) outputs signature \sigma
// outputs PS signatures on (m_1,...,m_n)
fn Sign(SecretKey : sk,  ) -> PSSignature {
    let x = sk(x);
    let sigma_tilde_1 = VerkeyGroup::from_msg_hash(&[label, " : g_tilde".as_bytes()].concat());
    let sumcalc = 0;
    for i in 0..n{
        let y_i = sk(y[i]);
        sumcalc += y_i; //need to add message
    }
    let sigma_tilde_2 = sigma_tilde_1 * (x*sumcalc);

    (1, 1, sigma_tilde_1, sigma_tilde_2)
}

fn Derive(PublicKey: pk, PSSignature: sigma, ){
    let sigma_prime_1 = ;
    let sigma_prime_2 =;

}