// API for CurveTree based simple accumulator

struct AccumulatorError {
}

// A struct to hold the data of a membership proof
struct AccumulatorMemberProof {
// Returns: FAIL (proof is invalid)
//      True (element is in accumulator)
//      False (element is not in accumulator -- but NonMemberProof may be necessary as the two proofs
// should be quite different)
    element;
    zkdata;
}

// A struct to hold the data of a membership proof
struct AccumulatorNonMemberProof();
// Returns: FAIL (proof is invalid)
//      True (element is not in accumulator)
//      False (element is in accumulator -- this is probably impossible and MemberProof is better)

// A struct holding the accumulator root (e.g. Merkle root or curve point)
struct AccumulatorRoot();

// An accumulator that holds the root and can check proofs but accumulator is fixed
trait AccumulatorVerifier {
    fn constructor(&self, root);
}

// FIXME use Result<T, E> everywhere?

trait AccumulatorCreator {
    fn add(&self, element);
    fn add(&self, Vec<element>);
    fn remove(&self, element); // Maybe don't have this
    fn has(&self, element); // Check assuming the Accumulator has all elements in the clear
    fn membership_proof(&self, element) -> AccumulatorProof;
    fn nonmembership_proof(&self, element) -> AccumulatorProof;
    fn check_member(&self, AccumulatorMemberProof) -> Result<AccumulatorMemberProof, AccumulatorError>; // Check proof of membership
    fn check_nonmember(&self, AccumulatorNonMemberProof) -> Result<AccumulatorNonMemberProof, AccumulatorError>;
    fn get_root(&self) -> AccumulatorRoot;
}

// A struct holding an accumulator and all elements
struct CurveTreeAccumulator();

impl Accumulator for CurveTreeAccumulator {
    fn k
}
