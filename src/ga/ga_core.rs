// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett, carlos-lopez-garces
// rust-monster is licensed under an MIT License.

//! GA Core 
//! Defines the core traits to work with rust-monster


use ::ga::ga_population::{GAPopulation, GAPopulationSortOrder};
use ::ga::ga_random::GARandomCtx;

use std::any::Any;

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

/// Genetic Algorithm Individual
pub trait GAIndividual
{
    // Instance
    fn crossover(&self, other: &Self, rng_ctx: &mut GARandomCtx) -> Box<Self>;
    fn mutate(&mut self, pMutation: f32, rng_ctx: &mut GARandomCtx);
    fn evaluate(&mut self, evaluation_ctx: &mut Any);
    // Fitness score
    fn fitness(&self) -> f32;
    fn set_fitness(&mut self, f: f32);
    // Raw score
    fn raw(&self) -> f32;
    fn set_raw(&mut self, r: f32);
}


/// Genetic Algorithm Individual Factory
pub trait GAFactory<T: GAIndividual>
{
    // Create a population with n randomly initialized individuals
    fn random_population(&mut self, n: usize, sort_order: GAPopulationSortOrder,
                         rng_ctx: &mut GARandomCtx) -> GAPopulation<T>;
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
    fn population(&mut self) -> &mut GAPopulation<T>;

    fn initialize_internal(&mut self) {}
    fn step_internal(&mut self) -> i32 { 0 }
    fn done_internal(&mut self) -> bool { true }
}
