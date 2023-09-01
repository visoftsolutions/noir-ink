use ethnum::{U256, uint};

pub struct Point {
    pub x: U256,
    pub y: U256,
}

pub struct C {
    pub c0: U256,
    pub c1: U256,
}

pub struct G2X {
    pub x: C,
    pub y: C,
}

pub struct VerificationKey {
    pub circuit_size: U256,
    pub num_inputs: U256,
    pub work_root: U256,
    pub domain_inverse: U256,
    pub q1: Point,
    pub q2: Point,
    pub q3: Point,
    pub q4: Point,
    pub q_m: Point,
    pub q_c: Point,
    pub q_arithmetic: Point,
    pub qsort: Point,
    pub q_elliptic: Point,
    pub q_aux: Point,
    pub sigma1: Point,
    pub sigma2: Point,
    pub sigma3: Point,
    pub sigma4: Point,
    pub table1: Point,
    pub table2: Point,
    pub table3: Point,
    pub table4: Point,
    pub table_type: Point,
    pub id1: Point,
    pub id2: Point,
    pub id3: Point,
    pub id4: Point,
    pub contains_recursive_proof: U256,
    pub recursive_proof_public_input_indices: U256,
    pub g2_x: G2X,
    pub work_root_inverse: U256,
}

pub fn load_verification_key() -> VerificationKey {
    VerificationKey {
        circuit_size: uint!("0x0000000000000000000000000000000000000000000000000000000000000010"),
        num_inputs: uint!("0x0000000000000000000000000000000000000000000000000000000000000001"),
        work_root: uint!("0x21082ca216cbbf4e1c6e4f4594dd508c996dfbe1174efb98b11509c6e306460b"),
        domain_inverse: uint!("0x2d5e098bb31e86271ccb415b196942d755b0a9c3f21dd9882fa3d63ab1000001"),
        q1: Point { 
            x: uint!("0x17c660f12bd58e45a1161d687125bcf3e50db901cd031ae90fa0f899c5f6b0af"),
            y: uint!("0x18431833019d0b6edd508ef905b14dc5489f7392ae6ac1e78785eeaf36dc198e"),
        },
        q2: Point { 
            x: uint!("0x0e0b6d8dd8f37229da396ea6af4d0950760cb8b36391ed9babf2816005d98431"),
            y: uint!("0x023976dee62b72746539eca15a1b803d7aa47b5e13143d496e96b90fb399ac70"),
        },
        q3: Point {
            x: uint!("0x2a7e71e447b5645910a429e7f48f1a5deba7f7d446b95a5edd242b55f67993d3"),
            y: uint!("0x2b1ea7f7453a8c80a89a675245da0c33db05ba8e95ecea432ab85f6b2d6a1e86"),
        },
        q4: Point {
            x: uint!("0x02d6fd9e84dbe74b7531e1801405a1c292117b1a17fefe9de0bfd9edf1a84bf9"),
            y: uint!("0x293c6ab3c06a0669af13393a82c60a459a3b2a0b768da45ac7af7f2aec40fc42"),
        },
        q_m: Point {
            x: uint!("0x0efe5ad29f99fce939416b6638dff26c845044cca9a2d9dbf94039a11d999aaa"),
            y: uint!("0x0a44bf49517a4b66ae6b51eee6ac68587f768022c11ac8e37cd9dce243d01ef2"),
        },
        q_c: Point {
            x: uint!("0x117d457bfb28869ab380fd6e83133eeb5b6ab48e5df1ae9bc204b60817006655"),
            y: uint!("0x2a958a537a99428a1019fd2c8d6b97c48f3e74ad77f0e2c63c9dfb6dccf9a29c"),
        },
        q_arithmetic: Point {
            x: uint!("0x18c3e78f81e83b52719158e4ac4c2f4b6c55389300451eb2a2deddf244129e7a"),
            y: uint!("0x0002e9c902fe5cd49b64563cadf3bb8d7beb75f905a5894e18d27c42c62fd797"),
        },
        qsort: Point {
            x: uint!("0x2cbce7beee3076b78dace04943d69d0d9e28aa6d00e046852781a5f20816645c"),
            y: uint!("0x2bc27ec2e1612ea284b08bcc55b6f2fd915d11bfedbdc0e59de09e5b28952080"),
        },
        q_elliptic: Point {
            x: uint!("0x0ad34b5e8db72a5acf4427546c7294be6ed4f4d252a79059e505f9abc1bdf3ed"),
            y: uint!("0x1e5b26790a26eb340217dd9ad28dbf90a049f42a3852acd45e6f521f24b4900e"),
        },
        q_aux: Point {
            x: uint!("0x155a0f51fec78c33ffceb7364d69d7ac27e570ae50bc180509764eb3fef94815"),
            y: uint!("0x1c1c4720bed44a591d97cbc72b6e44b644999713a8d3c66e9054aa5726324c76"),
        },
        sigma1: Point {
            x: uint!("0x0f7261cf55a71f4d0d7b961dda9ddb058cc5ad51753faec2a5908155d472e429"),
            y: uint!("0x1b7b1a10c1e638ce11d8c84b831aca4a36df78f0d50144437ef26f8bbfe69ac1"), 
        },
        sigma2: Point {
            x: uint!("0x163a9c8b67447afccc64e9ccba9d9e826ba5b1d1ddd8d6bb960f01cd1321a169"),
            y: uint!("0x19256311d43dbc795f746c63b209667653a773088aba5c6b1337f435188d72c4"), 
        },
        sigma3: Point {
            x: uint!("0x1fa51a0d75363b3af4e259e0dbb2c5df58a7bad9afe3651be67bc6c298092e11"),
            y: uint!("0x21915198840ad9c3666122b2837aeac8b5836b29551d41dbc04bdb1fcf1a1868"), 
        },
        sigma4: Point {
            x: uint!("0x0cee6b75dcf02a07c50939e8ca3cf35df0e69d7efdbc7898b3762f0a0ed976ad"),
            y: uint!("0x27e49262bd388ce2d0f193988f3b8f66a493be1ea69d2b335152719acd54d735"), 
        },
        table1: Point {
            x: uint!("0x02c397073c8abce6d4140c9b961209dd783bff1a1cfc999bb29859cfb16c46fc"),
            y: uint!("0x2b7bba2d1efffce0d033f596b4d030750599be670db593af86e1923fe8a1bb18"),
        },
        table2: Point {
            x: uint!("0x2c71c58b66498f903b3bbbda3d05ce8ffb571a4b3cf83533f3f71b99a04f6e6b"),
            y: uint!("0x039dce37f94d1bbd97ccea32a224fe2afaefbcbd080c84dcea90b54f4e0a858f"),
        },
        table3: Point {
            x: uint!("0x27dc44977efe6b3746a290706f4f7275783c73cfe56847d848fd93b63bf32083"),
            y: uint!("0x0a5366266dd7b71a10b356030226a2de0cbf2edc8f085b16d73652b15eced8f5"),
        },
        table4: Point {
            x: uint!("0x136097d79e1b0ae373255e8760c49900a7588ec4d6809c90bb451005a3de3077"),
            y: uint!("0x13dd7515ccac4095302d204f06f0bff2595d77bdf72e4acdb0b0b43969860d98"),
        },
        table_type: Point {
            x: uint!("0x16ff3501369121d410b445929239ba057fe211dad1b706e49a3b55920fac20ec"),
            y: uint!("0x1e190987ebd9cf480f608b82134a00eb8007673c1ed10b834a695adf0068522a"),
        },
        id1: Point {
            x: uint!("0x1e44194e60f0ab4ee0f77adc50f4220944f94301aa6da3016a226de04de52f4c"),
            y: uint!("0x2a017d0d9f40d0aeb5c8152ffddec56c2c7bea37dfbd20be6bed19efd743397a"),
        },
        id2: Point {
            x: uint!("0x27579be0883627093cf8bdec0b72e77f43efe5631bf48c872c317bed3b8bf12b"),
            y: uint!("0x0ddb2d01ec88ed69144177a4af3850cef6108b89e89b35679431d113f3be7dff"),
        },
        id3: Point {
            x: uint!("0x0a7fe830f1cb7a5d49d71877dd226a0083e7ab1f26781948b36d131759f7c8c9"),
            y: uint!("0x2db7a5185064e6501ef61e989895a01834ecd1ce1e8e80812bdd95f960a45e57"),
        },
        id4: Point {
            x: uint!("0x2eea648c8732596b1314fe2a4d2f05363f0c994e91cecad25835338edee2294f"),
            y: uint!("0x0ab49886c2b94bd0bd3f6ed1dbbe2cb2671d2ae51d31c1210433c3972bb64578"),
        },
        contains_recursive_proof: uint!("0x00"),
        recursive_proof_public_input_indices: uint!("0x00"),
        g2_x: G2X {
            x: C {
                c0: uint!("0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1"),
                c1: uint!("0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0"),
            },
            y: C {
                c0: uint!("0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4"),
                c1: uint!("0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55"),
            },
        },
        work_root_inverse: uint!("0x02e40daf409556c02bfc85eb303402b774954d30aeb0337eb85a71e6373428de"),
    }
}