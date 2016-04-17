// TODO: COPYRIGHT, USE & AUTHORS
// TODO: RUST DOCS!

extern crate rust_monster;
#[macro_use]
extern crate log;
extern crate env_logger;
#[cfg(test)]
mod tests
{
    use rust_monster::ga::ga_core::{GeneticAlgorithm, GASolution, GAFactory, DEBUG_FLAG};
    use rust_monster::ga::ga_population::{GAPopulation, GAPopulationSortOrder, GAPopulationSortBasis};
    use rust_monster::ga::ga_simple::{SimpleGeneticAlgorithm, SimpleGeneticAlgorithmCfg};
    use rust_monster::ga::ga_selectors::{GASelector, GARankSelector, GAUniformSelector, GARouletteWheelSelector, GATournamentSelector, GARawScoreBasedSelection, GAScaledScoreBasedSelection};
    use rust_monster::ga::ga_random;

    use env_logger;
    use std::sync::{Once, ONCE_INIT};

    static INIT: Once = ONCE_INIT;
    const VAL: f32 = 3.14159;
    
    struct TestSolution
    {
        fitness: f32
    }
    impl GASolution for TestSolution 
    {
        fn new() -> TestSolution
        {
            TestSolution{ fitness: VAL }
        }

        fn clone(&self) -> Self { TestSolution::new() }
        fn evaluate(&mut self) -> f32 { self.fitness }
    #[allow(unused_variables)]
        fn crossover(&self, other : &Self) -> Self { TestSolution::new() }
    #[allow(unused_variables)]
        fn mutate(&mut self, pm : f32) {}
        fn fitness(&self) -> f32 { (1.0 / self.fitness) }
        fn score(&self) -> f32 { self.fitness }
    }

    #[allow(dead_code)]
    struct TestFactory
    {
        starting_fitness: f32
    }
    impl GAFactory<TestSolution> for TestFactory
    {
        fn initial_population(&mut self) -> GAPopulation<TestSolution> {
            GAPopulation::new(vec![TestSolution { fitness: self.starting_fitness }], GAPopulationSortOrder::HighIsBest)
        }
    }

////////////////////////////////////////
// Utility functions

    fn ga_test_setup()
    {
        // This is only called once, works nicely as a Setup mechanism
        // TODO: Can we do somethign similar for teardown?
        INIT.call_once(||{
            env_logger::init().unwrap();
        });
    }


    fn simple_ga_validation(sga:&mut SimpleGeneticAlgorithm<TestSolution>)
    {
        sga.initialize();
        assert_eq!(sga.step(), 1);
        assert_eq!(sga.done(), false);
        assert_eq!(sga.population().size(), 1);
        assert_eq!(sga.population().best().score(), VAL);
    }

////////////////////////////////////////
// Tests
    
    #[test]
    fn init_test_with_initial_population()
    {
        ga_test_setup();
        let initial_population = GAPopulation::new(vec![TestSolution { fitness: VAL}], GAPopulationSortOrder::HighIsBest);
        let mut ga : SimpleGeneticAlgorithm<TestSolution> =
                     SimpleGeneticAlgorithm::new(SimpleGeneticAlgorithmCfg {
                                                   d_seed : 1,
                                                   flags : DEBUG_FLAG,
                                                   max_generations: 100,
                                                   ..Default::default()
                                                 },
                                                 None,
                                                 Some(initial_population) 
                                                 );
        simple_ga_validation(&mut ga);
    }

    #[test]
    fn init_test_with_factory()
    {
        ga_test_setup();
        let mut factory = TestFactory { starting_fitness: VAL };
        let mut ga : SimpleGeneticAlgorithm<TestSolution> =
                     SimpleGeneticAlgorithm::new(SimpleGeneticAlgorithmCfg {
                                                   d_seed : 1,
                                                   flags : DEBUG_FLAG,
                                                   max_generations: 100,
                                                   ..Default::default()
                                                 },
                                                 Some(&mut factory as &mut GAFactory<TestSolution>),
                                                 None
                                                 );
        simple_ga_validation(&mut ga);
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn init_test_missing_args()
    {
        ga_test_setup();
        let ga : SimpleGeneticAlgorithm<TestSolution> =
                 SimpleGeneticAlgorithm::new(SimpleGeneticAlgorithmCfg {
                                               d_seed : 1,
                                               flags : DEBUG_FLAG,
                                               max_generations: 100,
                                               ..Default::default()
                                             },
                                             None,
                                             None 
                                             );
        // Not reached
    }

    #[test]
    #[should_panic]
    fn init_test_empty_initial_pop()
    {
        ga_test_setup();
        let empty_initial_population : GAPopulation<TestSolution> = GAPopulation::new(vec![], GAPopulationSortOrder::HighIsBest);
        let mut ga : SimpleGeneticAlgorithm<TestSolution> =
                     SimpleGeneticAlgorithm::new(SimpleGeneticAlgorithmCfg {
                                                   d_seed : 1,
                                                   flags : DEBUG_FLAG,
                                                   max_generations: 100,
                                                   ..Default::default()
                                                 },
                                                 None,
                                                 Some(empty_initial_population) 
                                                 );
        ga.initialize()
        //Not reached 
    }

    // This test should live in the ga_population file directly,
    // but since it requires GASolution objects it is here for now.
    // Maybe we could use a Mocking library.
    // TODO: This doesn't go here.
    #[test]
    fn test_sort_population()
    {
        let f = VAL;
        let f_m = VAL - 1.0;
        let i_f = 1.0 / f;
        let i_f_m = 1.0 / f_m;

        let mut population = GAPopulation::new(vec![TestSolution { fitness: f }, TestSolution { fitness: f_m }], GAPopulationSortOrder::HighIsBest);
        population.sort();

        //TestSolution's Fitness is the inverse of the Score (F = 1/S)
        assert_eq!(population.individual(0, GAPopulationSortBasis::Raw).score(), f);
        assert_eq!(population.individual(1, GAPopulationSortBasis::Raw).score(), f_m);
        assert_eq!(population.individual(0, GAPopulationSortBasis::Scaled).fitness(), i_f_m);
        assert_eq!(population.individual(1, GAPopulationSortBasis::Scaled).fitness(), i_f);
    }

    #[test]
    #[allow(unused_variables)]
    fn test_rank_selector()
    {
        let f = VAL;
        let f_m = VAL - 1.0;
        let i_f = 1.0 / f;
        let i_f_m = 1.0 / f_m;

        let mut population
          = GAPopulation::new(vec![TestSolution { fitness: f },
                                   TestSolution { fitness: f_m }],
                              GAPopulationSortOrder::HighIsBest);

        {
            let raw_score_selection = GARawScoreBasedSelection;
            let mut raw_rank_selector = GARankSelector::new(&mut population, &raw_score_selection);

            raw_rank_selector.update();

            // Best Raw score is that of 1st solution.
            assert_eq!(raw_rank_selector.select().score(), f);
        }

        {
            let scaled_score_selection = GAScaledScoreBasedSelection;
            let mut scaled_rank_selector = GARankSelector::new(&mut population, &scaled_score_selection);

            scaled_rank_selector.update();

            // Best Scaled score is that of 2nd solution (because fitness is inverse of score). Weird. In this case, LowIsBest.
            assert_eq!(scaled_rank_selector.select().fitness(), i_f_m);
        }
    }

    #[test]
    fn test_uniform_selector()
    {
        let f = VAL;
        let f_m = VAL - 1.0;

        let mut population
          = GAPopulation::new(vec![TestSolution { fitness: f },
                                   TestSolution { fitness: f_m }],
                              GAPopulationSortOrder::HighIsBest);

        let mut uniform_selector = GAUniformSelector::new(&mut population);

        uniform_selector.update();

        let selected_individual = uniform_selector.select();
        assert!(selected_individual.score() == f || selected_individual.score() == f_m);  
    }

    #[test]
    #[allow(unused_variables)]
    fn test_roulette_wheel_selector()
    {
        // Just exercise the code.
        // TODO: How to test when there is randomness?

        let mut individuals = vec![];

        for i in 1 .. 20
        {
            individuals.push(TestSolution { fitness: ga_random::ga_random_float() });
        }

        let mut population
          = GAPopulation::new(individuals, GAPopulationSortOrder::LowIsBest);

        {
            let raw_score_selection = GARawScoreBasedSelection;

            let mut raw_roulette_wheel_selector 
              = GARouletteWheelSelector::new(&mut population, &raw_score_selection);

            raw_roulette_wheel_selector.update();

            raw_roulette_wheel_selector.select();
        }

        {
            let scaled_score_selection = GAScaledScoreBasedSelection;

            let mut scaled_roulette_wheel_selector 
              = GARouletteWheelSelector::new(&mut population, &scaled_score_selection);

            scaled_roulette_wheel_selector.update();

            scaled_roulette_wheel_selector.select();
        }
    }

    #[test]
    #[allow(unused_variables)]
    fn test_tournament_selector()
    {
        // Just exercise the code.
        // TODO: How to test when there is randomness?

        let mut individuals = vec![];

        for i in 1 .. 20
        {
            individuals.push(TestSolution { fitness: ga_random::ga_random_float() });
        }

        let mut population
          = GAPopulation::new(individuals, GAPopulationSortOrder::LowIsBest);

        {
            let raw_score_selection = GARawScoreBasedSelection;

            let mut raw_tournament_selector 
              = GARouletteWheelSelector::new(&mut population, &raw_score_selection);

            raw_tournament_selector.update();

            raw_tournament_selector.select();
        }

        {
            let scaled_score_selection = GAScaledScoreBasedSelection;

            let mut scaled_tournament_selector 
              = GARouletteWheelSelector::new(&mut population, &scaled_score_selection);

            scaled_tournament_selector.update();

            scaled_tournament_selector.select();
        }
    }

}
