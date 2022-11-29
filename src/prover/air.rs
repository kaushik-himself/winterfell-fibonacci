use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    Air, AirContext, Assertion, ByteWriter, EvaluationFrame, ProofOptions, Serializable, TraceInfo,
    TransitionConstraintDegree,
};

// Public inputs for our computation will consist of the starting value and the end result.
// pub struct PublicInputs {
//     start: BaseElement,
//     result: BaseElement,
// }

// // We need to describe how public inputs can be converted to bytes.
// impl Serializable for PublicInputs {
//     fn write_into<W: ByteWriter>(&self, target: &mut W) {
//         target.write(self.start);
//         target.write(self.result);
//     }
// }

// impl PublicInputs {
//     pub fn new(start: BaseElement, result: BaseElement) -> Self {
//         Self { start, result }
//     }
// }

// For a specific instance of our computation, we'll keep track of the public inputs and
// the computation's context which we'll build in the constructor. The context is used
// internally by the Winterfell prover/verifier when interpreting this AIR.
pub struct WorkAir {
    context: AirContext<BaseElement>,
    result: BaseElement,
}

impl Air for WorkAir {
    // First, we'll specify which finite field to use for our computation, and also how
    // the public inputs must look like.
    type BaseField = BaseElement;
    type PublicInputs = BaseElement;

    // Here, we'll construct a new instance of our computation which is defined by 3 parameters:
    // starting value, number of steps, and the end result. Another way to think about it is
    // that an instance of our computation is a specific invocation of the do_work() function.
    fn new(trace_info: TraceInfo, pub_input: Self::BaseField, options: ProofOptions) -> Self {
        assert_eq!(2, trace_info.width());

        let degrees = vec![
            TransitionConstraintDegree::new(1),
            TransitionConstraintDegree::new(1),
        ];

        let num_assertions = 3;

        WorkAir {
            context: AirContext::new(trace_info, degrees, num_assertions, options),
            result: pub_input,
        }
    }

    // In this method we'll define our transition constraints; a computation is considered to
    // be valid, if for all valid state transitions, transition constraints evaluate to all
    // zeros, and for any invalid transition, at least one constraint evaluates to a non-zero
    // value. The `frame` parameter will contain current and next states of the computation.
    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();

        result[0] = next[0] - current[0] - current[1];
        result[1] = next[1] - current[1] - next[0];
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, Self::BaseField::ONE),
            Assertion::single(1, 0, Self::BaseField::ONE),
            Assertion::single(1, last_step, self.result),
        ]
    }

    // This is just boilerplate which is used by the Winterfell prover/verifier to retrieve
    // the context of the computation.
    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
}
