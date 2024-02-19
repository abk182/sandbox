export const getTargetIncome = (currentIncome, years, indexing = true) => {
    const inflation =  0.05;
    let currentYear = 0;
    let calculatedIncome = currentIncome;
    while (currentYear < years) {
        if (indexing) {
            calculatedIncome = calculatedIncome * (1.0 + inflation);
        }
        currentYear += 1;
    }

    return calculatedIncome;
}