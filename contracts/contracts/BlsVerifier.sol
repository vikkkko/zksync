pragma solidity ^0.5.8;

import "./BlsOperations.sol";

contract BlsVerifier {

    function aggregatePubkeys(BlsOperations.G2Point[] memory _pubkeys) internal view returns (BlsOperations.G2Point memory) {
        require(
            _pubkeys.length > 0,
            "osas1"
        ); // osas1 - pubkeys count must be more than 0
        BlsOperations.G2Point memory aggrPubkey;
        if (_pubkeys.length == 1) {
            aggrPubkey = _pubkeys[0];
        } else {
            for (uint256 i = 0; i < _pubkeys.length; i++) {
                aggrPubkey = BlsOperations.addG2(aggrPubkey, _pubkeys[i]);
            }
        }
        return aggrPubkey;
    }

    function aggregateSignatures(BlsOperations.G1Point[] memory _signatures) internal view returns (BlsOperations.G1Point memory) {
        require(
            _signatures.length > 0,
            "osas2"
        ); // osas2 - signatures count must be more than 0
        BlsOperations.G1Point memory aggrSignature;
        if (_signatures.length == 1) {
            aggrSignature = _signatures[0];
        } else {
            for (uint256 i = 0; i < _signatures.length; i++) {
                aggrSignature = BlsOperations.addG1(aggrSignature, _signatures[i]);
            }
        }
        return aggrSignature;
    }

    function verifyBlsSignature(
        BlsOperations.G1Point memory _signature,
        BlsOperations.G2Point[] memory _pubKeys,
        bytes memory _message
    ) internal view returns (bool) {
        BlsOperations.G1Point memory mpoint = BlsOperations.messageToG1(_message);
        BlsOperations.G2Point memory aggrPubkey = aggregatePubkeys(_pubKeys);
        return BlsOperations.pairing(mpoint, aggrPubkey, _signature, BlsOperations.negate(BlsOperations.generatorG2()));
    }
}