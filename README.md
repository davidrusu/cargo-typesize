# cargo typesize

List the size of all types in a Rust crate

## Install + Usage

Cargo typesize depends on nightly for building and running, this is a restriction imposed by rustc to prevent linking against the rustc compiler on stable.

```
# Add the necessary components to the nightly compiler
rustup default nightly
rustup component add rust-src rustc-dev llvm-tools-preview
```

```
# Install cargo-typesize using the nightly compiler
cargo +nightly install cargo-typesize
```

Then make sure all invocations are done under nightly:

```
cargo +nightly typesize
```


## Sample output
Output of running `cargo typesize` on [sn_consensus](https://github.com/maidsafe/sn_consensus)

```
Inspecting layout of lib: sn_consensus
  0	() - src/decision.rs:9:56: 9:65 (#247)
  0	() - src/decision.rs:9:67: 9:78 (#258)
  0	() - src/fault.rs:19:56: 19:65 (#340)
  0	() - src/fault.rs:19:67: 19:78 (#353)
  0	() - src/sn_membership.rs:23:55: 23:64 (#408)
  0	() - src/sn_membership.rs:23:66: 23:77 (#411)
  0	() - src/vote.rs:13:49: 13:58 (#458)
  0	() - src/vote.rs:13:60: 13:71 (#467)
  0	() - src/vote.rs:186:49: 186:58 (#545)
  0	() - src/vote.rs:186:60: 186:71 (#556)
  0	() - src/vote.rs:88:49: 88:58 (#499)
  0	() - src/vote.rs:88:60: 88:71 (#510)
  0	<fault::_::<impl decision::_::_serde::Deserialize<'de> for fault::Fault<T>>::deserialize::__Visitor<'de, T> as decision::_::_serde::de::Visitor<'de>>::visit_enum::__FieldVisitor - src/fault.rs:19:67: 19:78 (#353)
  0	<fault::_::<impl decision::_::_serde::Deserialize<'de> for fault::Fault<T>>::deserialize::__Visitor<'de, T> as decision::_::_serde::de::Visitor<'de>>::visit_enum::__Visitor<'de, T> - src/fault.rs:19:67: 19:78 (#353)
  0	<vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Ballot<T>>::deserialize::__Visitor<'de, T> as decision::_::_serde::de::Visitor<'de>>::visit_enum::__FieldVisitor - src/vote.rs:13:60: 13:71 (#467)
  0	<vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Ballot<T>>::deserialize::__Visitor<'de, T> as decision::_::_serde::de::Visitor<'de>>::visit_enum::__Visitor<'de, T> - src/vote.rs:13:60: 13:71 (#467)
  0	decision::_::<impl decision::_::_serde::Deserialize<'de> for decision::Decision<T>>::deserialize::__FieldVisitor - src/decision.rs:9:67: 9:78 (#258)
  0	decision::_::<impl decision::_::_serde::Deserialize<'de> for decision::Decision<T>>::deserialize::__Visitor<'de, T> - src/decision.rs:9:67: 9:78 (#258)
  0	fault::_::<impl decision::_::_serde::Deserialize<'de> for fault::Fault<T>>::deserialize::__FieldVisitor - src/fault.rs:19:67: 19:78 (#353)
  0	fault::_::<impl decision::_::_serde::Deserialize<'de> for fault::Fault<T>>::deserialize::__Visitor<'de, T> - src/fault.rs:19:67: 19:78 (#353)
  0	sn_membership::_::<impl decision::_::_serde::Deserialize<'de> for sn_membership::Reconfig<T>>::deserialize::__FieldVisitor - src/sn_membership.rs:23:66: 23:77 (#411)
  0	sn_membership::_::<impl decision::_::_serde::Deserialize<'de> for sn_membership::Reconfig<T>>::deserialize::__Visitor<'de, T> - src/sn_membership.rs:23:66: 23:77 (#411)
  0	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Ballot<T>>::deserialize::__FieldVisitor - src/vote.rs:13:60: 13:71 (#467)
  0	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Ballot<T>>::deserialize::__Visitor<'de, T> - src/vote.rs:13:60: 13:71 (#467)
  0	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::SignedVote<T>>::deserialize::__FieldVisitor - src/vote.rs:186:60: 186:71 (#556)
  0	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::SignedVote<T>>::deserialize::__Visitor<'de, T> - src/vote.rs:186:60: 186:71 (#556)
  0	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Vote<T>>::deserialize::__FieldVisitor - src/vote.rs:88:60: 88:71 (#510)
  0	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Vote<T>>::deserialize::__Visitor<'de, T> - src/vote.rs:88:60: 88:71 (#510)
  1	<fault::_::<impl decision::_::_serde::Deserialize<'de> for fault::Fault<T>>::deserialize::__Visitor<'de, T> as decision::_::_serde::de::Visitor<'de>>::visit_enum::__Field - src/fault.rs:19:67: 19:78 (#353)
  1	<vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Ballot<T>>::deserialize::__Visitor<'de, T> as decision::_::_serde::de::Visitor<'de>>::visit_enum::__Field - src/vote.rs:13:60: 13:71 (#467)
  1	decision::_::<impl decision::_::_serde::Deserialize<'de> for decision::Decision<T>>::deserialize::__Field - src/decision.rs:9:67: 9:78 (#258)
  1	fault::FaultError - src/fault.rs:8:1: 17:2 (#0)
  1	fault::_::<impl decision::_::_serde::Deserialize<'de> for fault::Fault<T>>::deserialize::__Field - src/fault.rs:19:67: 19:78 (#353)
  1	sn_membership::_::<impl decision::_::_serde::Deserialize<'de> for sn_membership::Reconfig<T>>::deserialize::__Field - src/sn_membership.rs:23:66: 23:77 (#411)
  1	u8 - src/lib.rs:37:1: 37:22 (#0)
  1	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Ballot<T>>::deserialize::__Field - src/vote.rs:13:60: 13:71 (#467)
  1	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::SignedVote<T>>::deserialize::__Field - src/vote.rs:186:60: 186:71 (#556)
  1	vote::_::<impl decision::_::_serde::Deserialize<'de> for vote::Vote<T>>::deserialize::__Field - src/vote.rs:88:60: 88:71 (#510)
  8	u64 - src/sn_handover.rs:11:1: 11:32 (#0)
  8	u64 - src/sn_membership.rs:13:1: 13:27 (#0)
  8	usize - src/sn_membership.rs:12:1: 12:35 (#0)
 16	&'static [&'static str] - src/decision.rs:9:67: 9:78 (#258)
 16	&'static [&'static str] - src/fault.rs:19:67: 19:78 (#353)
 16	&'static [&'static str] - src/sn_membership.rs:23:66: 23:77 (#411)
 16	&'static [&'static str] - src/vote.rs:13:60: 13:71 (#467)
 16	&'static [&'static str] - src/vote.rs:186:60: 186:71 (#556)
 16	&'static [&'static str] - src/vote.rs:88:60: 88:71 (#510)
 24	error::Error - src/error.rs:7:1: 62:2 (#0)
 32	vote_count::SuperMajorityCount - src/vote_count.rs:60:1: 64:2 (#0)
 48	vote_count::Candidate<T> - src/vote_count.rs:12:1: 17:2 (#0)
 72	vote_count::VoteCount<T> - src/vote_count.rs:67:1: 71:2 (#0)
336	decision::Decision<T> - src/decision.rs:10:1: 14:2 (#0)
488	consensus::Consensus<T> - src/consensus.rs:12:1: 20:2 (#0)
496	sn_handover::Handover<T> - src/sn_handover.rs:14:1: 17:2 (#0)
544	sn_membership::Membership<T> - src/sn_membership.rs:16:1: 21:2 (#0)
```
