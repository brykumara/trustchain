use crate::subject::IONSubject;
use crate::TrustchainIONError;
use did_ion::sidetree::Sidetree;
use did_ion::sidetree::{DIDStatePatch, PublicKeyJwk, ServiceEndpointEntry};
use did_ion::ION;
use serde_json::{Map, Value};
use ssi::did::{Document, ServiceEndpoint};
use ssi::did_resolve::DocumentMetadata;
use ssi::jwk::{Base64urlUInt, ECParams, Params, JWK};
use std::convert::TryFrom;
use thiserror::Error;
use trustchain_core::controller::{Controller, ControllerError};
use trustchain_core::key_manager::{ControllerKeyManager, KeyManager, KeyManagerError, KeyType};
use trustchain_core::subject::{Subject, SubjectError};

impl KeyManager for IONController {}
impl ControllerKeyManager for IONController {}

/// Type for holding controller data.
struct ControllerData {
    did: String,
    controlled_did: String,
    update_key: JWK,
    recovery_key: JWK,
}

impl ControllerData {
    fn new(did: String, controlled_did: String, update_key: JWK, recovery_key: JWK) -> Self {
        ControllerData {
            did,
            controlled_did,
            update_key,
            recovery_key,
        }
    }
}

impl TryFrom<ControllerData> for IONController {
    type Error = Box<dyn std::error::Error>;
    fn try_from(data: ControllerData) -> Result<Self, Self::Error> {
        let controller = IONController {
            did: data.did,
            controlled_did: data.controlled_did,
        };
        // Save the update key
        controller.save_key(
            &controller.controlled_did,
            KeyType::UpdateKey,
            &data.update_key,
        )?;
        // Save the recovery key
        controller.save_key(
            &controller.controlled_did,
            KeyType::RecoveryKey,
            &data.recovery_key,
        )?;
        Ok(controller)
    }
}

/// Struct for common IONController.
pub struct IONController {
    did: String,
    controlled_did: String,
    // update_key: Option<JWK>,
    // recovery_key: Option<JWK>,
    // next_update_key: Option<JWK>,
}

impl IONController {
    /// Construct a new IONController instance
    /// from existing Subject and Controller DIDs.
    pub fn new(did: &str, controlled_did: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Returns a result with propagating error

        // Construct a KeyManager for the Subject.
        let subject = IONSubject::new(did);

        // // Construct a KeyManager for the Controller.
        // let update_key: Option<JWK> = match self.read_update_key(controlled_did) {
        //     Ok(x) => Some(x),
        //     Err(_) => {
        //         return Err(Box::new(ControllerError::NoUpdateKey(
        //             controlled_did.to_string(),
        //         )))
        //     }
        // };
        // let recovery_key: Option<JWK> = match self.read_recovery_key(controlled_did) {
        //     Ok(x) => Some(x),
        //     Err(_) => {
        //         return Err(Box::new(ControllerError::NoRecoveryKey(
        //             controlled_did.to_string(),
        //         )))
        //     }
        // };

        Ok(Self {
            did: did.to_owned(),
            controlled_did: controlled_did.to_owned(),
        })
    }

    /// Assume that the document to be made into a ION DID is agreed
    /// with subject (i.e. content is correct and subject has private key
    /// for public key in doc). The function then converts the document into
    /// a create operation that can be pushed to the ION server.
    fn create_subject(doc: Document) -> IONController {
        todo!()
    }
}

impl Subject for IONController {
    fn did(&self) -> &str {
        // TODO: consider whether happy with controlled_did being the "did" of
        // "controller"
        &self.did
    }
    fn attest(&self, doc: &Document, signing_key: &JWK) -> Result<String, SubjectError> {
        todo!()
    }
}

impl Controller for IONController {
    fn controlled_did(&self) -> &str {
        &self.controlled_did
    }

    fn update_key(&self) -> Result<JWK, KeyManagerError> {
        let update_key = self.read_update_key(self.controlled_did())?;
        Ok(update_key)
    }

    fn next_update_key(&self) -> Result<Option<JWK>, KeyManagerError> {
        let next_update_key = self.read_next_update_key(self.controlled_did())?;
        Ok(Some(next_update_key))
    }

    fn generate_next_update_key(&self) {
        todo!()
    }

    fn recovery_key(&self) -> Result<JWK, KeyManagerError> {
        let recovery_key = self.read_recovery_key(self.controlled_did())?;
        Ok(recovery_key)
    }

    fn into_subject(&self) -> Box<dyn Subject> {
        Box::new(IONSubject::new(&self.did))
    }
}

impl IONController {
    /// Checks whether there is a proof field in document metadata.
    pub fn is_proof_in_doc_meta(&self, doc_meta: &DocumentMetadata) -> bool {
        if let Some(property_set) = doc_meta.property_set.as_ref() {
            property_set.contains_key(&"proof".to_string())
        } else {
            false
        }
    }

    /// Function to return a patch for adding a proof service.
    pub fn add_proof_service(&self, did: &str, proof: &str) -> DIDStatePatch {
        let mut obj: Map<String, Value> = Map::new();
        obj.insert("controller".to_string(), Value::from(did));
        obj.insert("proofValue".to_string(), Value::from(proof.to_owned()));

        DIDStatePatch::AddServices {
            services: vec![ServiceEndpointEntry {
                id: "trustchain-controller-proof".to_string(),
                r#type: "TrustchainProofService".to_string(),
                service_endpoint: ServiceEndpoint::Map(serde_json::Value::Object(obj.clone())),
            }],
        }
    }

    /// Function to confirm whether a given key is the `commitment` in document metadata
    pub fn is_commitment_key(
        &self,
        doc_meta: &DocumentMetadata,
        key: &JWK,
        key_type: KeyType,
    ) -> bool {
        if let Ok(expected_commitment) = self.key_to_commitment(key) {
            if let Ok(actual_commitment) = self.extract_commitment(doc_meta, key_type) {
                actual_commitment == expected_commitment
            } else {
                // TODO: handle error
                panic!()
            }
        } else {
            // TODO: handle error
            panic!()
        }
    }

    /// Extracts commitment of passed key type from document metadata.s
    fn extract_commitment(
        &self,
        doc_meta: &DocumentMetadata,
        key_type: KeyType,
    ) -> Result<String, TrustchainIONError> {
        todo!()
    }

    /// Converts a given JWK into a commitment.
    fn key_to_commitment(&self, next_update_key: &JWK) -> Result<String, TrustchainIONError> {
        // https://docs.rs/did-ion/latest/src/did_ion/sidetree.rs.html#L214
        // 1. Convert next_update_key to public key (pk)
        // 2. Get commitment value from the pk
        // 3. Return value
        match &PublicKeyJwk::try_from(next_update_key.to_public()) {
            Ok(pk_jwk) => match ION::commitment_scheme(pk_jwk) {
                Ok(commitment) => Ok(commitment),
                Err(_) => Err(TrustchainIONError::FailedToConvertToCommitment),
            },
            Err(_) => Err(TrustchainIONError::FailedToConvertToCommitment),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use ssi::did::Proof;
    use ssi::did_resolve::DocumentMetadata;
    use trustchain_core::data::{
        TEST_NEXT_UPDATE_KEY, TEST_RECOVERY_KEY, TEST_SIGNING_KEYS, TEST_UPDATE_KEY,
    };
    use trustchain_core::data::{
        TEST_SIDETREE_DOCUMENT_METADATA, TEST_TRUSTCHAIN_DOCUMENT_METADATA,
    };
    use trustchain_core::init;

    // TODO: move the update_key and recovery_key loads out as lazy_static!()

    // Make a IONController using this test function
    fn test_controller(
        did: &str,
        controlled_did: &str,
    ) -> Result<IONController, Box<dyn std::error::Error>> {
        let update_key: JWK = serde_json::from_str(TEST_UPDATE_KEY)?;
        let recovery_key: JWK = serde_json::from_str(TEST_RECOVERY_KEY)?;
        IONController::try_from(ControllerData::new(
            did.to_string(),
            controlled_did.to_string(),
            update_key,
            recovery_key,
        ))
    }

    #[test]
    fn test_try_from() -> Result<(), Box<dyn std::error::Error>> {
        init();
        assert_eq!(0, 0);
        let update_key: JWK = serde_json::from_str(TEST_UPDATE_KEY)?;
        let recovery_key: JWK = serde_json::from_str(TEST_RECOVERY_KEY)?;
        let did = "did_try_from";
        let controlled_did = "controlled_did_try_from";

        // Make controller using try_from()
        let target = test_controller(did, controlled_did)?;

        assert_eq!(target.controlled_did(), controlled_did);

        let loaded_update_key = target.update_key()?;
        assert_eq!(loaded_update_key, update_key);

        let loaded_recovery_key = target.recovery_key()?;
        assert_eq!(loaded_recovery_key, recovery_key);

        Ok(())
    }

    #[test]
    fn test_into_subject() -> Result<(), Box<dyn std::error::Error>> {
        init();
        let did = "did_into_subject";
        let controlled_did = "controlled_did_into_subject";
        let target = test_controller(did, controlled_did)?;
        assert_eq!(target.did(), did);
        assert_ne!(target.did(), controlled_did);

        let result = target.into_subject();
        assert_eq!(result.did(), did);
        assert_ne!(result.did(), controlled_did);
        Ok(())
    }

    #[test]
    fn test_is_proof_in_doc_meta() -> Result<(), Box<dyn std::error::Error>> {
        init();
        let did = "did_is_proof_in_doc_meta";
        let controlled_did = "controlled_is_proof_in_doc_meta";
        let controller = test_controller(did, controlled_did)?;

        let tc_doc_meta: DocumentMetadata =
            serde_json::from_str(TEST_TRUSTCHAIN_DOCUMENT_METADATA)?;
        assert!(controller.is_proof_in_doc_meta(&tc_doc_meta));

        let sidetree_doc_meta: DocumentMetadata =
            serde_json::from_str(TEST_SIDETREE_DOCUMENT_METADATA)?;
        assert!(!controller.is_proof_in_doc_meta(&sidetree_doc_meta));

        Ok(())
    }

    #[test]
    fn test_extract_commitment() -> Result<(), Box<dyn std::error::Error>> {
        init();
        let did = "did_is_proof_in_doc_meta";
        let controlled_did = "controlled_is_proof_in_doc_meta";
        let controller = test_controller(did, controlled_did)?;
        let expected_recovery_commitment = "EiBKWQyomumgZvqiRVZnqwA2-7RVZ6Xr-cwDRmeXJT_k9g";
        let expected_update_commitment = "EiCe3q-ZByJnzI6CwGIDj-M67W-Yv78L3ejxcuEDxnWzMg";
        let doc_meta: DocumentMetadata = serde_json::from_str(TEST_TRUSTCHAIN_DOCUMENT_METADATA)?;

        let update_commiment = controller.extract_commitment(&doc_meta, KeyType::UpdateKey)?;
        assert_eq!(expected_update_commitment, update_commiment.as_str());

        let next_update_commiment =
            controller.extract_commitment(&doc_meta, KeyType::NextUpdateKey)?;
        assert_eq!(expected_update_commitment, next_update_commiment.as_str());

        let recovery_commiment = controller.extract_commitment(&doc_meta, KeyType::RecoveryKey)?;
        assert_eq!(expected_recovery_commitment, recovery_commiment.as_str());
        Ok(())
    }

    #[test]
    fn test_key_to_commitment() -> Result<(), Box<dyn std::error::Error>> {
        init();
        let did = "did_key_to_commitment";
        let controlled_did = "controlled_key_to_commitment";
        let update_key: JWK = serde_json::from_str(TEST_UPDATE_KEY)?;
        let recovery_key: JWK = serde_json::from_str(TEST_RECOVERY_KEY)?;

        let controller = test_controller(did, controlled_did)?;

        let expected_recovery_commitment = "EiBKWQyomumgZvqiRVZnqwA2-7RVZ6Xr-cwDRmeXJT_k9g";
        let expected_update_commitment = "EiCe3q-ZByJnzI6CwGIDj-M67W-Yv78L3ejxcuEDxnWzMg";

        let update_commitment = controller.key_to_commitment(&update_key)?;
        let recovery_commitment = controller.key_to_commitment(&recovery_key)?;

        assert_eq!(expected_update_commitment, update_commitment);
        assert_eq!(expected_recovery_commitment, recovery_commitment);

        Ok(())
    }

    #[test]
    fn test_is_commitment_key() -> Result<(), Box<dyn std::error::Error>> {
        init();
        let did = "did_is_commitment_key";
        let controlled_did = "controlled_is_commitment_key";
        let update_key: JWK = serde_json::from_str(TEST_UPDATE_KEY)?;
        let recovery_key: JWK = serde_json::from_str(TEST_RECOVERY_KEY)?;
        let controller = test_controller(did, controlled_did)?;
        let doc_meta: DocumentMetadata = serde_json::from_str(TEST_TRUSTCHAIN_DOCUMENT_METADATA)?;

        assert!(controller.is_commitment_key(&doc_meta, &update_key, KeyType::UpdateKey));
        assert!(controller.is_commitment_key(&doc_meta, &recovery_key, KeyType::RecoveryKey));
        Ok(())
    }

    #[test]
    fn test_add_proof_service() -> Result<(), Box<dyn std::error::Error>> {
        init();
        let did = "did_add_proof_service";
        let controlled_did = "controlled_add_proof_service";
        let controller = test_controller(did, controlled_did)?;
        let proof = "test_proof_information".to_string();
        let _ = controller.add_proof_service(controlled_did, &proof);
        Ok(())
    }
}
