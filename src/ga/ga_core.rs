// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett, carlos-lopez-garces
// rust-monster is licensed under an MIT License.

//! GA Core 
//! Defines the core traits to work with rust-monster


use super::ga_population::{GAPopulation, GAPopulationSortOrder};

/// Bit Flags for Genetic Algorithm Configuration 
/// 
///
bitflags!
{
    pub flags GAFlags: u32
    {
        const DEBUG_FLAG = 0b00000001
    }
}
impl Default for GAFlags
{
    fn default() -> GAFlags { GAFlags {bits : 0} }
}


/// Genetic Algorithm Configuration
///
/// 
pub trait GAConfig
{
    fn flags(&self) -> GAFlags;
    fn max_generations(&self) -> i32;
    fn probability_crossover(&self) -> f32;
    fn probability_mutation(&self) -> f32;
}


/// Genetic Algorithm Individual
pub trait GAIndividual
{
    //Static
    fn new(f:f32) -> Self;

    // Instance
    fn crossover(&self, other : &Self) -> Self;
    fn mutate(&mut self, pMutation : f32);
    fn evaluate(&mut self) -> f32;
    // Fitness score
    fn fitness(&self) -> f32;
    fn set_fitness(&mut self, f : f32);
    // Raw score
    fn raw(&self) -> f32;
}


/// Genetic Algorithm Individual Factory
pub trait GAFactory<T: GAIndividual>
{
    fn initial_population(&mut self) -> GAPopulation<T> 
    {
        GAPopulation::new(vec![], GAPopulationSortOrder::HighIsBest)
    }
}


/// Genetic Algorithm
pub trait GeneticAlgorithm<T: GAIndividual>
{
    // GENERIC GA METHODS - Should not be overriden frequently
    fn initialize(&mut self)
    {
        debug!("Genetic Algorithm - Initialized");
        self.initialize_internal()
    }

    fn step(&mut self) -> i32
    { 
        debug!("Genetic Algorithm - Step");
        self.step_internal()
    }

    fn done(&mut self) -> bool
    {
        debug!("Genetic Algorithm - Done");
        self.done_internal()
    }

    // IMPLEMENTATION SPECIFIC
    fn config(&mut self) -> &GAConfig;

    fn population(&mut self) -> &mut GAPopulation<T>;

    fn initialize_internal(&mut self) {}
    fn step_internal(&mut self) -> i32 { 0 }
    fn done_internal(&mut self) -> bool { true }
}
