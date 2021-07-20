#[cfg(test)]
pub const SAMPLE_V1: &str = r#"{
	"gen": "srtool v0.9.14",
	"src": "git",
	"version": "0.9.7",
	"commit": "5d35bac7408a4cb12a578764217d06f3920b36aa",
	"tag": "v0.9.7-rc3",
	"branch": "heads/v0.9.7-rc3",
	"rustc": "rustc 1.53.0 (53cb7b09b 2021-06-17)",
	"pkg": "polkadot-runtime",
	"tmsp": "2021-06-29T16:12:59Z",
	"size": "2093380",
	"prop": "0x424ac5063ce844b878cd418e7d4c0e5518a6323ec0c54f744b1fb44a2ab24dcd",
	"ipfs": "QmeBgekBhZHNCkrayDgQaLXfAoLibS5Eq2fyEv4rzbttTo",
	"sha256": "0x5f31cd25a9de645f278f18b008f38edad5b3253c1b94dc71a12da48c27dd1581",
	"wasm": "runtime/polkadot/target/srtool/release/wbuild/polkadot-runtime/polkadot_runtime.compact.wasm"
}"#;

#[cfg(test)]
pub const SAMPLE_V2: &str = r#"{
    "info": {
	  "generator": {
		"name": "srtool",
		"version": "0.9.15"
	  },
	  "src": "git",
	  "version": "0.9.7",
	  "git": {
		"commit": "5d35bac7408a4cb12a578764217d06f3920b36aa",
		"tag": "v0.9.7-rc3",
		"branch": "heads/v0.9.7-rc3"
	  },
	  "rustc": "rustc 1.53.0 (53cb7b09b 2021-06-17)",
	  "pkg": "polkadot-runtime",
	  "profile": "release"
	},
	"context": {
	  "package": "polkadot-runtime",
	  "runtime_dir": "runtime/polkadot",
	  "docker": {
		"image": "chevdor/srtool",
		"tag": "1.53.0"
	  },
	  "profile": "release"
	},
	"runtimes": {
	  "compact": {
		"tmsp": "2021-06-29T16:12:24Z",
		"sha256": "0x5f31cd25a9de645f278f18b008f38edad5b3253c1b94dc71a12da48c27dd1581",
		"wasm": "runtime/polkadot/target/srtool/release/wbuild/polkadot-runtime/polkadot_runtime.compact.wasm",
		"subwasm": {
		  "size": 2093380,
		  "compression": {
			"size_compressed": 2093380,
			"size_decompressed": 2093380,
			"compressed": false
		  },
		  "reserved_meta_valid": true,
		  "metadata_version": 13,
		  "core_version": "polkadot-9070 (parity-polkadot-0.tx7.au0)",
		  "proposal_hash": "0x424ac5063ce844b878cd418e7d4c0e5518a6323ec0c54f744b1fb44a2ab24dcd",
		  "ipfs_hash": "QmeBgekBhZHNCkrayDgQaLXfAoLibS5Eq2fyEv4rzbttTo",
		  "blake2_256": "0xc5daf28ebf7f23c8de92a99a6c15b84abeaf12d226542e7504febaf0d1484e05"
		}
	  },
	  "compressed": {}
  }
}"#;

#[cfg(test)]
pub const SAMPLE_V3: &str = r#"{
    "version": "0.9.17",
    "info": {
	  "generator": {
		"name": "srtool",
		"version": "0.9.17"
	  },
	  "src": "git",
	  "version": "0.9.7",
	  "git": {
		"commit": "5d35bac7408a4cb12a578764217d06f3920b36aa",
		"tag": "v0.9.7-rc3",
		"branch": "heads/v0.9.7-rc3"
	  },
	  "rustc": "rustc 1.53.0 (53cb7b09b 2021-06-17)",
	  "pkg": "polkadot-runtime",
	  "profile": "release"
	},
	"context": {
	  "package": "polkadot-runtime",
	  "runtime_dir": "runtime/polkadot",
	  "docker": {
		"image": "chevdor/srtool",
		"tag": "1.53.0"
	  },
	  "profile": "release"
	},
	"runtimes": {
	  "compact": {
		"tmsp": "2021-06-29T16:12:24Z",
		"sha256": "0x5f31cd25a9de645f278f18b008f38edad5b3253c1b94dc71a12da48c27dd1581",
		"wasm": "runtime/polkadot/target/srtool/release/wbuild/polkadot-runtime/polkadot_runtime.compact.wasm",
		"subwasm": {
		  "size": 2093380,
		  "compression": {
			"size_compressed": 2093380,
			"size_decompressed": 2093380,
			"compressed": false
		  },
		  "reserved_meta_valid": true,
		  "metadata_version": 13,
		  "core_version": "polkadot-9070 (parity-polkadot-0.tx7.au0)",
		  "proposal_hash": "0x424ac5063ce844b878cd418e7d4c0e5518a6323ec0c54f744b1fb44a2ab24dcd",
		  "ipfs_hash": "QmeBgekBhZHNCkrayDgQaLXfAoLibS5Eq2fyEv4rzbttTo",
		  "blake2_256": "0xc5daf28ebf7f23c8de92a99a6c15b84abeaf12d226542e7504febaf0d1484e05"
		}
	  },
	  "compressed": {}
  }
}"#;

#[cfg(test)]
pub const SAMPLE_V4: &str = r#"{
    "V4": {
        "version": "1.2.3"
    }
}"#;
