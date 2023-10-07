
/*
    To calculate the Bayesian posterior probability, we need:
        - The prior probability of the hypothesis being true
        - The likelihood of the data given the hypothesis (if the hypothesis is true, how likely is the data?)
        - The probability of the evidence (independent of the hypothesis)
 */
fn calculate_bayesian_posterior(prior: f64, likelihood: f64, evidence: f64) -> f64 {
    let prob: f64 = (prior * likelihood) / evidence;
    return prob;
}

fn calculate_posterior_for_observation(hypothesis: &Hypothesis, observation: &Observation) -> f64 {
    let prob: f64 = (hypothesis.probability * observation.dependent_probability) / observation.independent_probability;
    return prob;
}

struct Hypothesis {
    probability: f64,
}

struct Observation {
    // Probability of the observation given the hypothesis
    dependent_probability: f64,
    // Probability of the observation independent of the hypothesis
    independent_probability: f64,
}


fn main() {
    let prior = 0.01;
    let likelihood = 0.90;
    let data = 0.2;

    let posterior = calculate_bayesian_posterior(prior, likelihood, data);
    // println!("The posterior probability is: {}", posterior);

    let mut hype1 = Hypothesis {
        probability: 0.01
    };

    let obs1 = Observation {
        dependent_probability: 0.9,
        independent_probability: 0.2,
    };

    hype1.probability = calculate_posterior_for_observation(&hype1, &obs1);
    hype1.probability = calculate_posterior_for_observation(&hype1, &obs1);

    /*
    Where this gets interesting is when we want to calculate things that have inverse probabilities.
    For example, if it rains on Monday, we know it won't rain on Tuesday. So, given the probability
    of rain on Monday, what's the chance that Tuesday will be sunny?
     */

    let mut mondayRain: Hypothesis = Hypothesis{ probability: 0.25 };

    let mut tuesdaySunny: Hypothesis = Hypothesis { probability: mondayRain.probability };

    println!("{}", tuesdaySunny.probability);
    mondayRain.probability = 0.50;
    println!("{}", tuesdaySunny.probability);
}

#[cfg(test)]
mod tests {
    use crate::{calculate_bayesian_posterior, Hypothesis, Observation, calculate_posterior_for_observation};

    #[test]
    fn test_bayesian_raw_calculation() {
        let prior = 0.05;
        let likelihood = 0.85;
        let data = 0.25;

        let posterior = calculate_bayesian_posterior(prior, likelihood, data);
        assert_eq!(posterior, 0.17);
    }

    #[test]
    fn test_bayesian_model() {
        let hypothesis: Hypothesis = Hypothesis { probability: 0.05 };
        let observation: Observation = Observation { 
            dependent_probability: 0.85,
            independent_probability: 0.25 
        };

        let posterior: f64 = calculate_posterior_for_observation(&hypothesis, &observation);
        assert_eq!(posterior, 0.17);
    }
}
