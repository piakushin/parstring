use crate::expression::Expression;

#[test]
fn acceptance_criteria() {
    let inputs = [
        "3a2c4",
        "32a2d2",
        "500a10b66c32",
        "3ae4c66fb32",
        "3c4d2aee2a4c41fc4f",
    ];
    let outputs = [20, 17, 14208, 235, 990];
    inputs
        .iter()
        .map(|input| {
            println!("{input}");
            Expression::try_from(input).unwrap().evaluate().unwrap().0
        })
        .zip(outputs)
        .for_each(|(output, correct)| assert_eq!(output, correct));
}
