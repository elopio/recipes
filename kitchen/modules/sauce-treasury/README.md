# smpl-treasury

This is a simple implementation of the treasury. It is designed for teaching purposes and is intended to be dead simple with minimal associated logic. For this reason, this code is very insecure and we will discuss the vulnerabilities in what follows.

## no anti-sybil mechanism on vote

## no cleaning of `Proposals` storage item (periodic flush)

## iteration in the runtime is a big NO
* but either way, these spends need to be executed
* is there a more efficient pattern for batching consecutive spends?

## `treasury_spend`
* reorder the proposals based on support (could also configure to be optimize for paying off as many as possible)


## extensions

add to `treasury_spend`, something like

```rust
// take a slice of proposals with a certain age
let required_age = 10;
let now = <system::Module<T>>::block_number();
let mut old_enough_proposals = Vec::new();
<Council<T>>::get().into_iter(|member| {
    if let Some(proposal) = <Proposals<T>>::get(member) {
        if now - proposal.when > required_age {
            old_enough_proposals.push(proposal);
        }
    }
});

/// Order based on `support` field
///
/// - used in unstable sort in `treasury_spend()`
impl Ord for Proposal<T::AccountId, T::Balance, T::BlockNumber> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.support.cmp(&other.support)
    }
}
```

The point might be to sort the treasury proposals based on how much support they received for example...