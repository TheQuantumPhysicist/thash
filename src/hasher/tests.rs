use std::collections::BTreeMap;

use crate::{hasher::make_hasher, options::HashAlgorithm};

#[test]
fn sha1() {
    let expected = [
        "a9993e364706816aba3e25717850c26c9cd0d89d",
        "0d3ced9bec10a777aec23ccc353a8c08a633045e",
        "65ba90ac47c0e09f83e6257ca0453aeb19df63cf",
        "11b407ae7f9e7c783a17f0f8458e6e26adffe766",
        "b5c64925eb9940259be55c005c9cecc7d9897ef9",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha1,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn sha224() {
    let expected = [
        "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7",
        "2f6268de8b61017569d68593c00a7b442f9dcb898743f4262be4d444",
        "ec0a402c15e68c1e65c974fa42583e215946e4611e7dac3433b4d30a",
        "ca37eefe9b151642a52f255311c3f665a53b71403e4ee7493d930e19",
        "5b4b17f720d52c6a864229e784fb636184ca48ce7dd848fdad986239",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha224,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn sha256() {
    let expected = [
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        "4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358",
        "f2a778f1a6ed3d5bc59a5d79104c598f3f07093f240ca4e91333fb09ed4f36da",
        "ebea187d3d64ec287600c6be94f0db8ab5b5ff8382b6ac4a45218e6e5b327c7f",
        "184f6d6e82554c051b33f15e7ffffecb0cc0f461a29096c41c214e168e34c21d",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha256,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn sha384() {
    let expected = [
        "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7",
        "73100f01cf258766906c34a30f9a486f07259c627ea0696d97c4582560447f59a6df4a7cf960708271a30324b1481ef4",
        "dc786f9fab31a3a0b51f3e277daa9874d33c0950e7aaf53df3b01c7abae1161e7556aeed36c6c11ec9381cbff81a7261",
        "f7d89a51fe0d9091c6f9297a580d57930aed700435865d8d89d8d73fa424a936cd47c34c294013406e8fd369803e5232",
        "a4aa4cd8534aecb2d07765f928303d1d2609835ea85d14312bcee264e99dc5d7dc08bb18ec694053fd7fe6906706d55f",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha384,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn sha512() {
    let expected = [
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        "373a9f3a902cf561003b513c94c5164ba4af135cbc4eb4d856b89ea5609523f130bbe5e453e6c645b2765a265aaeb1390c82c913130870636cd0c8ecf980d851",
        "f3ea3dce496e48ccb10e68c1ce4e9d9eb77601054e1bb70469fcf61c58c6d28fd53b34c84b21b4ea6431d5bffbe57d360f6f119db1abf4aa35f85378c3b691ee",
        "3b360d3e5e0e0d0e4136f0304c5a966acbe4aa543b9e140f0b6df7c95455c4fdfa1ca876b30d9ef7720da00cd063223f7f0dfd05003bfb3ad0751684d824fab0",
        "299b2e3ce932e4d0e9005345e37af5a4cc6be21e6b6e21231ce71ccde2a7aba4a6822cd7a9aaf9b13918db05ede70d3f1e6af65f8ad0bda1c4c4fa263e3cabdd",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha512,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn sha3_224() {
    let expected = [
        "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf",
        "c18edd254284eee2ac9b82f293cc6a535f66c21059d49074f29361cb",
        "15377eefd5d4e4c9269dc191963338ac43517aa5e78c31ef852edf8e",
        "4caea5749bb056384ab2e0599b387b1e0f8c8114069091bd23e29712",
        "7d208060760d239d9e9b041b5c30ac992b83ff1df658263953c9eff0",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha3_224,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn sha3_256() {
    let expected = [
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532",
        "f6362cbb9fb8a60f03c2f0d8124d2c6a1a828e2db8e8b05a6f699735b4492cbc",
        "cd2416183a8fca0d97e778722f027b35b3ee820016435c6d1473cf6baf41dcd8",
        "75abe2e20f0454c7c83f186e79ee38bc3579dd6ae770c9bdc24f26e7918df53a",
        "fd5ad48a1abf3fd8211ecd2a6a0b0503e745d953def260541fa5db7dc1b3b84f",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha3_256,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn sha3_384() {
    let expected = [
        "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25",
        "7365d226caddaa8ec62faa1537886da61396b7507c2b99d4a244a17a3ad8174a6ee92c191ac2f6da78d4b5990c026195",
        "48b76d3df955b2944b2e9fb568dfec2dd0b6486bc9e553e7381f57cd2a3ff59feef5dcd55a9a1081a50f22366f506e25",
        "b3be88ebaf0d9813ed54caede7ea33b7c1aaf0dc1881731f30b0dfe8befd70a1c50d81a8ce269fde09a5a1fd8fbb16b8",
        "be2f2365cecd5df751f3ab7d23cabfb60491ce28bdf80b121f7941ee33227ce86d5d62d6633f5654a4f3ae5381cf1825",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha3_384,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn sha3_512() {
    let expected = [
        "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0",
        "465558b627e37552639af5d20d59fdfe150016d40b97b7d0cb66420d86585d82461e2eda3295903357bcb0a5b67c07aaf561d7e9ee193480095291af9b2132e1",
        "0706ea1e63053d8f6a83c8c01a70edebb4cffa23d3ea889bd28a96dfcdfc5bedc8d82fb5b6362f13507347008040e3b0b49a05ddf7502ad124983b02faf54b73",
        "22d91b012bc405ff59a23fd851a5146fca3a9c6aa018bea6a35f875ea5e82b141048261a7736d753d4ddfd4528324bee7403595a4c02992f501690bed36bf75e",
        "8c74189ca608ad188bb96c8c374fb717ce982500dc2c0ce90ad8e5888b498ce9fda0e4bf256feeaaf1674b69e9ea80cf5ed444dfdd5d3eb05cfebd597b4aab67",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Sha3_512,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn blake2b() {
    let expected = [
        "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923",
        "66cb547665e462bbdd51d9b6ce1221116e9cfc6711c78d8798158349d12fa8ca513efb14bd84edf4e7cd3551355f14c1cf54dd203669b95675e52d72d3ec00d9",
        "2ddda015a6b31d39fa9e6d54bb55bab1999a224d23b094fb1f77c41a1ea597c485e10bc721dd5531f1cddc52fdafa09c03ac4fbaaac9271241bd1da64dbd390c",
        "50f4b533357084ec5a41ff26dfd36e069a1bf23ed6fd17ee341cf082d409854480332831399565d3f6fa0bed4cab0fad7c81c62b66c2b328ab880f139a094e1c",
        "500cb0c9c086a7d65309a6e1d792501f811812411dc22f557c687af44428b68ce19f15ffe1f469cad0fe1180182151ac86f7f406f97e35f943bb084f1f51462b",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Blake2b,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn blake2s() {
    let expected = [
        "508c5e8c327c14e2e1a72ba34eeb452f37458b209ed63a294d999b4c86675982",
        "4fc81429a06db63df17d1b4a9ea1592b2ceb48443e136cd255b2c3f737a88283",
        "d35cd060f4eac30b9cc6795b781ef69e404afcf28938811e3e77564b4e065c77",
        "6ba692e8584769323fc63ad295823868d48bdbb85f468e3c19f20cf63c9a00ce",
        "261be591f339375efa8fc6b929235873f6e7dab1e9ed7cdc7421e7071e7d5c59",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Blake2s,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn blake3() {
    // To generate, you can cut parts from the command below
    // echo -n "abc" | ./thash -a blake3 -F binary | ./thash -a blake3 -F binary | ./thash -a blake3 -F binary | ./thash -a blake3 -F binary | b3sum
    let expected = [
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
        "dc2f738d17b8a7ec03efdd0a95e8d3924b8e05965040ab6dcafd992994012bd4",
        "b908276b102aa54b21e9c7b84f34cf8efda591157c153cdf73f186a5005ab649",
        "815db8711bd9bd1c759ca340c3279d59fb43c51d3a9a2a975691f72f000122a7",
        "acfec95e0441b70ebce15e924c3d2edacd40d5b2495dc24eb5d3e4ad2f29ca9b",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Blake3,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn md5() {
    let expected = [
        "900150983cd24fb0d6963f7d28e17f72",
        "af5da9f45af7a300e3aded972f8ff687",
        "8f354f270114bc87029306db6b405e63",
        "50bff3827c8a17192ed5a879fbf9ee70",
        "e2753218c2dfa2487b258c6868cc8cbe",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::Md5,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}

#[test]
fn k12() {
    // To generate, you can cut parts from the command below
    // echo -n "abc" | ./thash -a k12 -F binary | ./thash -a k12 -F binary | ./thash -a k12 -F binary | ./thash -a k12 -F binary | k12sum
    let expected = [
        "ab174f328c55a5510b0b209791bf8b60e801a7cfc2aa42042dcb8f547fbe3a7d",
        "412504d0ab2972e65c81662d471f1f1eccfa840be8dbb68fd9da753c580b3341",
        "52f45f653d2cdadc16f32e370520e468cd3b9bf8a23f14b82a1e090bf52b8dae",
        "919da7a0538fe1dd0f2b2d3289627002180038f22c7be1676ac91ed9fc740804",
        "7a79becf8062f604d557f4472f8098678b8c02ac7febbc5c1af9ed5bf9bcbbd2",
    ];
    let opts = BTreeMap::new();
    for (i, el) in expected.into_iter().enumerate() {
        let mut hasher = make_hasher(
            HashAlgorithm::K12,
            (i as u64 + 1).try_into().unwrap(),
            &opts,
        )
        .unwrap();
        hasher.write(b"abc");
        assert_eq!(hex::encode(hasher.finalize_and_reset()), el);
    }
}
