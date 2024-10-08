#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status

compare_output() {
    if ! diff -q <(echo "$1") <(echo "$2") >/dev/null; then
        echo "Test failed: Output mismatch"
        echo "LHS: $1"
        echo "RHS: $2"
        exit 1
    fi
}

cargo build --release
cargo install rdgen


######################################################################
# Random tests
######################################################################

######################################################################
output1=$(echo -n "abc" | ./target/release/thash)
output2=$(echo "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256)
output2=$(echo "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -F hex-lower)
output2=$(echo "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -F hex-upper)
output2=$(echo "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -F base64)
output2=$(echo "ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0=")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -F base64-no-pad)
output2=$(echo "ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -F base64-url-safe)
output2=$(echo "ungWv48Bz-pBQUDeXa4iI7ADYaOWF3qctBD_YfIAFa0=")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -F base64-url-safe-no-pad)
output2=$(echo "ungWv48Bz-pBQUDeXa4iI7ADYaOWF3qctBD_YfIAFa0")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -i2)
output2=$(echo "4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -F binary | target/release/thash -a sha256)
output2=$(echo "4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -F binary | target/release/thash -a blake2s -F binary | target/release/thash -a sha3-224)
output2=$(echo "c287a7a4abc221b50aa406b6b6e47017f0bb5bc354870912fc00588d")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -o output-size=64)
output2=$(echo "ab174f328c55a5510b0b209791bf8b60e801a7cfc2aa42042dcb8f547fbe3a7d3f5b54d116a705d36aac2a7eac7a19e3f0f058cb3c238ac7f034178ae34f212e")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA1
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha1)
output2=$(echo "a9993e364706816aba3e25717850c26c9cd0d89d")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha1 -i1)
output2=$(echo "a9993e364706816aba3e25717850c26c9cd0d89d")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha1 -i2)
output2=$(echo "0d3ced9bec10a777aec23ccc353a8c08a633045e")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha1 -i3)
output2=$(echo "65ba90ac47c0e09f83e6257ca0453aeb19df63cf")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha1 -i4)
output2=$(echo "11b407ae7f9e7c783a17f0f8458e6e26adffe766")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha1 -i5)
output2=$(echo "b5c64925eb9940259be55c005c9cecc7d9897ef9")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA224
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha224)
output2=$(echo "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha224 -i1)
output2=$(echo "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha224 -i2)
output2=$(echo "2f6268de8b61017569d68593c00a7b442f9dcb898743f4262be4d444")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha224 -i3)
output2=$(echo "ec0a402c15e68c1e65c974fa42583e215946e4611e7dac3433b4d30a")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha224 -i4)
output2=$(echo "ca37eefe9b151642a52f255311c3f665a53b71403e4ee7493d930e19")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha224 -i5)
output2=$(echo "5b4b17f720d52c6a864229e784fb636184ca48ce7dd848fdad986239")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA256
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256)
output2=$(echo "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -i1)
output2=$(echo "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -i2)
output2=$(echo "4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -i3)
output2=$(echo "f2a778f1a6ed3d5bc59a5d79104c598f3f07093f240ca4e91333fb09ed4f36da")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -i4)
output2=$(echo "ebea187d3d64ec287600c6be94f0db8ab5b5ff8382b6ac4a45218e6e5b327c7f")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha256 -i5)
output2=$(echo "184f6d6e82554c051b33f15e7ffffecb0cc0f461a29096c41c214e168e34c21d")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA384
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha384)
output2=$(echo "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha384 -i1)
output2=$(echo "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha384 -i2)
output2=$(echo "73100f01cf258766906c34a30f9a486f07259c627ea0696d97c4582560447f59a6df4a7cf960708271a30324b1481ef4")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha384 -i3)
output2=$(echo "dc786f9fab31a3a0b51f3e277daa9874d33c0950e7aaf53df3b01c7abae1161e7556aeed36c6c11ec9381cbff81a7261")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha384 -i4)
output2=$(echo "f7d89a51fe0d9091c6f9297a580d57930aed700435865d8d89d8d73fa424a936cd47c34c294013406e8fd369803e5232")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha384 -i5)
output2=$(echo "a4aa4cd8534aecb2d07765f928303d1d2609835ea85d14312bcee264e99dc5d7dc08bb18ec694053fd7fe6906706d55f")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA512
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha512)
output2=$(echo "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha512 -i1)
output2=$(echo "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha512 -i2)
output2=$(echo "373a9f3a902cf561003b513c94c5164ba4af135cbc4eb4d856b89ea5609523f130bbe5e453e6c645b2765a265aaeb1390c82c913130870636cd0c8ecf980d851")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha512 -i3)
output2=$(echo "f3ea3dce496e48ccb10e68c1ce4e9d9eb77601054e1bb70469fcf61c58c6d28fd53b34c84b21b4ea6431d5bffbe57d360f6f119db1abf4aa35f85378c3b691ee")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha512 -i4)
output2=$(echo "3b360d3e5e0e0d0e4136f0304c5a966acbe4aa543b9e140f0b6df7c95455c4fdfa1ca876b30d9ef7720da00cd063223f7f0dfd05003bfb3ad0751684d824fab0")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha512 -i5)
output2=$(echo "299b2e3ce932e4d0e9005345e37af5a4cc6be21e6b6e21231ce71ccde2a7aba4a6822cd7a9aaf9b13918db05ede70d3f1e6af65f8ad0bda1c4c4fa263e3cabdd")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA3-224
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-224)
output2=$(echo "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-224 -i1)
output2=$(echo "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-224 -i2)
output2=$(echo "c18edd254284eee2ac9b82f293cc6a535f66c21059d49074f29361cb")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-224 -i3)
output2=$(echo "15377eefd5d4e4c9269dc191963338ac43517aa5e78c31ef852edf8e")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-224 -i4)
output2=$(echo "4caea5749bb056384ab2e0599b387b1e0f8c8114069091bd23e29712")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-224 -i5)
output2=$(echo "7d208060760d239d9e9b041b5c30ac992b83ff1df658263953c9eff0")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA3-256
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-256)
output2=$(echo "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-256 -i1)
output2=$(echo "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-256 -i2)
output2=$(echo "f6362cbb9fb8a60f03c2f0d8124d2c6a1a828e2db8e8b05a6f699735b4492cbc")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-256 -i3)
output2=$(echo "cd2416183a8fca0d97e778722f027b35b3ee820016435c6d1473cf6baf41dcd8")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-256 -i4)
output2=$(echo "75abe2e20f0454c7c83f186e79ee38bc3579dd6ae770c9bdc24f26e7918df53a")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-256 -i5)
output2=$(echo "fd5ad48a1abf3fd8211ecd2a6a0b0503e745d953def260541fa5db7dc1b3b84f")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA3-384
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-384)
output2=$(echo "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-384 -i1)
output2=$(echo "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-384 -i2)
output2=$(echo "7365d226caddaa8ec62faa1537886da61396b7507c2b99d4a244a17a3ad8174a6ee92c191ac2f6da78d4b5990c026195")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-384 -i3)
output2=$(echo "48b76d3df955b2944b2e9fb568dfec2dd0b6486bc9e553e7381f57cd2a3ff59feef5dcd55a9a1081a50f22366f506e25")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-384 -i4)
output2=$(echo "b3be88ebaf0d9813ed54caede7ea33b7c1aaf0dc1881731f30b0dfe8befd70a1c50d81a8ce269fde09a5a1fd8fbb16b8")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-384 -i5)
output2=$(echo "be2f2365cecd5df751f3ab7d23cabfb60491ce28bdf80b121f7941ee33227ce86d5d62d6633f5654a4f3ae5381cf1825")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - SHA3-512
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-512)
output2=$(echo "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-512 -i1)
output2=$(echo "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-512 -i2)
output2=$(echo "465558b627e37552639af5d20d59fdfe150016d40b97b7d0cb66420d86585d82461e2eda3295903357bcb0a5b67c07aaf561d7e9ee193480095291af9b2132e1")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-512 -i3)
output2=$(echo "0706ea1e63053d8f6a83c8c01a70edebb4cffa23d3ea889bd28a96dfcdfc5bedc8d82fb5b6362f13507347008040e3b0b49a05ddf7502ad124983b02faf54b73")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-512 -i4)
output2=$(echo "22d91b012bc405ff59a23fd851a5146fca3a9c6aa018bea6a35f875ea5e82b141048261a7736d753d4ddfd4528324bee7403595a4c02992f501690bed36bf75e")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a sha3-512 -i5)
output2=$(echo "8c74189ca608ad188bb96c8c374fb717ce982500dc2c0ce90ad8e5888b498ce9fda0e4bf256feeaaf1674b69e9ea80cf5ed444dfdd5d3eb05cfebd597b4aab67")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - Blake2b
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2b)
output2=$(echo "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2b -i1)
output2=$(echo "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2b -i2)
output2=$(echo "66cb547665e462bbdd51d9b6ce1221116e9cfc6711c78d8798158349d12fa8ca513efb14bd84edf4e7cd3551355f14c1cf54dd203669b95675e52d72d3ec00d9")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2b -i3)
output2=$(echo "2ddda015a6b31d39fa9e6d54bb55bab1999a224d23b094fb1f77c41a1ea597c485e10bc721dd5531f1cddc52fdafa09c03ac4fbaaac9271241bd1da64dbd390c")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2b -i4)
output2=$(echo "50f4b533357084ec5a41ff26dfd36e069a1bf23ed6fd17ee341cf082d409854480332831399565d3f6fa0bed4cab0fad7c81c62b66c2b328ab880f139a094e1c")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2b -i5)
output2=$(echo "500cb0c9c086a7d65309a6e1d792501f811812411dc22f557c687af44428b68ce19f15ffe1f469cad0fe1180182151ac86f7f406f97e35f943bb084f1f51462b")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - Blake2s
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2s)
output2=$(echo "508c5e8c327c14e2e1a72ba34eeb452f37458b209ed63a294d999b4c86675982")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2s -i1)
output2=$(echo "508c5e8c327c14e2e1a72ba34eeb452f37458b209ed63a294d999b4c86675982")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2s -i2)
output2=$(echo "4fc81429a06db63df17d1b4a9ea1592b2ceb48443e136cd255b2c3f737a88283")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2s -i3)
output2=$(echo "d35cd060f4eac30b9cc6795b781ef69e404afcf28938811e3e77564b4e065c77")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2s -i4)
output2=$(echo "6ba692e8584769323fc63ad295823868d48bdbb85f468e3c19f20cf63c9a00ce")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake2s -i5)
output2=$(echo "261be591f339375efa8fc6b929235873f6e7dab1e9ed7cdc7421e7071e7d5c59")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - Blake3
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake3)
output2=$(echo "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake3 -i1)
output2=$(echo "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake3 -i2)
output2=$(echo "dc2f738d17b8a7ec03efdd0a95e8d3924b8e05965040ab6dcafd992994012bd4")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake3 -i3)
output2=$(echo "b908276b102aa54b21e9c7b84f34cf8efda591157c153cdf73f186a5005ab649")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake3 -i4)
output2=$(echo "815db8711bd9bd1c759ca340c3279d59fb43c51d3a9a2a975691f72f000122a7")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a blake3 -i5)
output2=$(echo "acfec95e0441b70ebce15e924c3d2edacd40d5b2495dc24eb5d3e4ad2f29ca9b")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - MD5
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a md5)
output2=$(echo "900150983cd24fb0d6963f7d28e17f72")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a md5 -i1)
output2=$(echo "900150983cd24fb0d6963f7d28e17f72")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a md5 -i2)
output2=$(echo "af5da9f45af7a300e3aded972f8ff687")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a md5 -i3)
output2=$(echo "8f354f270114bc87029306db6b405e63")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a md5 -i4)
output2=$(echo "50bff3827c8a17192ed5a879fbf9ee70")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a md5 -i5)
output2=$(echo "e2753218c2dfa2487b258c6868cc8cbe")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - K12
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12)
output2=$(echo "ab174f328c55a5510b0b209791bf8b60e801a7cfc2aa42042dcb8f547fbe3a7d")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i1)
output2=$(echo "ab174f328c55a5510b0b209791bf8b60e801a7cfc2aa42042dcb8f547fbe3a7d")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i2)
output2=$(echo "412504d0ab2972e65c81662d471f1f1eccfa840be8dbb68fd9da753c580b3341")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i3)
output2=$(echo "52f45f653d2cdadc16f32e370520e468cd3b9bf8a23f14b82a1e090bf52b8dae")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i4)
output2=$(echo "919da7a0538fe1dd0f2b2d3289627002180038f22c7be1676ac91ed9fc740804")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i5)
output2=$(echo "7a79becf8062f604d557f4472f8098678b8c02ac7febbc5c1af9ed5bf9bcbbd2")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - K12 - 64 bytes
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -o output-size=64)
output2=$(echo "ab174f328c55a5510b0b209791bf8b60e801a7cfc2aa42042dcb8f547fbe3a7d3f5b54d116a705d36aac2a7eac7a19e3f0f058cb3c238ac7f034178ae34f212e")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i1 -o output-size=64)
output2=$(echo "ab174f328c55a5510b0b209791bf8b60e801a7cfc2aa42042dcb8f547fbe3a7d3f5b54d116a705d36aac2a7eac7a19e3f0f058cb3c238ac7f034178ae34f212e")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i2 -o output-size=64)
output2=$(echo "1cdda776c6268dfad928a48e70786d0b20403bec42801315f752c5347ab73f9b8bc618015d597f9224785f1fc8254b80e1efa29333589551dd9e7807fb053349")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i3 -o output-size=64)
output2=$(echo "8e77fd9444f35c8a876eede9ee7d0f208dd245a40a65f0a5e1608c8912a6dd895eb8e615d7288978054a24c052c2d7774f20533890b385cd19ccf0ac46b39a25")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i4 -o output-size=64)
output2=$(echo "39d6703df1e0efefc212539c434c787f0e500ef2a59530ce1e390fef1bdd3ff6160d7bcb5c99562f21309722e792b99f53c15923393932e6c8c52b78cec67cc5")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | target/release/thash -a k12 -i5 -o output-size=64)
output2=$(echo "5a64ecb9f9b01b665b49fa6ebd4de4c73f5a2709e495555563fbddc9e9737771c71cd68613df48351ff321e73585e938bc4df6c1ebaa959547b165535146f05e")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - Large data with pipe
######################################################################

######################################################################
output1=$(echo -n "abc" | rdgen -l100000 | target/release/thash)
output2=$(echo "befb724c57435953a1740c0467f85ec09c212259e88afdd9a079c465ec03fd2a41ff976000d87f2f9c4eb87a0c05f5dc1be60c63650e2e8eb09b94faaff830c8")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | rdgen -l100000 | target/release/thash -i10)
output2=$(echo "95b0e2039c1e25b99c98a5d71537d3746020ccbeab017a99f9650ea912f93b8171d5035fed3c524425a9a6358bdd5615e847c90b7e57428d51ec11d2f90db1ca")

compare_output "$output1" "$output2"
######################################################################

######################################################################
output1=$(echo -n "abc" | rdgen -l100000 | target/release/thash -i10 -a sha256)
output2=$(echo "39ccf4908ef54473556e785c4eb248fe28bd229b00bb008a61c811d795966e6b")

compare_output "$output1" "$output2"
######################################################################


######################################################################
# Tests of algos - Large data with file
######################################################################

######################################################################
echo -n "abc" | rdgen -l100000 > data.bin
output1=$(target/release/thash -f data.bin)
output2=$(echo "befb724c57435953a1740c0467f85ec09c212259e88afdd9a079c465ec03fd2a41ff976000d87f2f9c4eb87a0c05f5dc1be60c63650e2e8eb09b94faaff830c8")

compare_output "$output1" "$output2"
######################################################################

######################################################################
echo -n "abc" | rdgen -l100000 > data.bin
output1=$(target/release/thash -f data.bin -i10)
output2=$(echo "95b0e2039c1e25b99c98a5d71537d3746020ccbeab017a99f9650ea912f93b8171d5035fed3c524425a9a6358bdd5615e847c90b7e57428d51ec11d2f90db1ca")

compare_output "$output1" "$output2"
######################################################################

######################################################################
echo -n "abc" | rdgen -l100000 > data.bin
output1=$(target/release/thash -f data.bin -i10 -a sha256)
output2=$(echo "39ccf4908ef54473556e785c4eb248fe28bd229b00bb008a61c811d795966e6b")

compare_output "$output1" "$output2"
######################################################################

rm data.bin


######################################################################
echo "All tests passed successfully."
