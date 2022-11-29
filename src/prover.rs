use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    FieldExtension, HashFunction, ProofOptions, Prover, StarkProof, Trace, TraceTable,
};

mod air;
use air::WorkAir;

pub fn build_do_work_trace(n: usize) -> TraceTable<BaseElement> {
    let trace_width = 2;
    let mut trace = TraceTable::new(trace_width, n / 2);

    trace.fill(
        |state| {
            state[0] = BaseElement::ONE;
            state[1] = BaseElement::ONE;
        },
        |_, state| {
            state[0] += state[1];
            state[1] += state[0];
        },
    );

    trace
}

// Our prover needs to hold STARK protocol parameters which are specified via ProofOptions
// struct.
struct WorkProver {
    options: ProofOptions,
}

impl WorkProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

// When implementing Prover trait we set the `Air` associated type to the AIR of the
// computation we defined previously, and set the `Trace` associated type to `TraceTable`
// struct as we don't need to define a custom trace for our computation.
impl Prover for WorkProver {
    type BaseField = BaseElement;
    type Air = WorkAir;
    type Trace = TraceTable<Self::BaseField>;

    // Our public inputs consist of the first and last value in the execution trace.
    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {
        let last_step = trace.length() - 1;
        trace.get(1, last_step)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}

pub fn prove_work() -> (BaseElement, StarkProof) {
    // Hardcoded base parameters for this example
    let n = 1_048_576;

    // Build the execution trace and get the result from the last step.
    let trace = build_do_work_trace(n);
    let result = trace.get(1, trace.length() - 1);

    // Define proof options; these will be enough for ~96-bit security level.
    let options = ProofOptions::new(
        32, // number of queries
        8,  // blowup factor
        0,  // grinding factor
        HashFunction::Blake3_256,
        FieldExtension::None,
        8,   // FRI folding factor
        128, // FRI max remainder length
    );

    // Instantiate the prover and generate the proof.
    let prover = WorkProver::new(options);
    let proof = prover.prove(trace).unwrap();

    (result, proof)
}

pub fn verify_work(result: BaseElement, proof: StarkProof) {
    match winterfell::verify::<WorkAir>(proof, result) {
        Ok(_) => println!("yay! all good!"),
        Err(_) => panic!("something went wrong while verifying the proof!"),
    }
}
