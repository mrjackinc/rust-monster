// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett
// rust-monster is licensed under a MIT License.

//! GA Test Utilities
//! Reusable classes for testing

use super::ga_core::*;
use super::ga_population::*;

#[cfg(test)]
extern crate env_logger;
pub const GA_TEST_FITNESS_VAL: f32 = 3.14159;

/// GA Test Setup
/// Utility function to setup useful test systems (like logging)
pub fn ga_test_setup(test_name: &str)
{
    ga_test_setup_internal(test_name);
}

#[cfg(not(test))]
fn ga_test_setup_internal(_: &str)
{
    // This only exists to avoid bringing env_logger in non-test builds
    // but keeping ga_test_setup documented
    // This is needed because [cfg(test)] conditional compilation wont
    // work on non-item statements as of rustc 1.9 (is experimental)
    // DO NOT ADD CODE HERE (See below)
}

#[cfg(test)]
fn ga_test_setup_internal(test_name: &str)
{
    let _ = env_logger::init();
    debug!("{:?}", test_name);
}


/// GA Test Teardown
/// Utlity function to teardown used test systems
pub fn ga_test_teardown(){}


/// GATestSolution
/// Implements the GASolution Trait with only no-ops
pub struct GATestSolution
{
    score: f32,
    fitness: f32
}
impl GASolution for GATestSolution 
{
    fn new(rs:f32) -> GATestSolution
    {
        GATestSolution{ score: rs, fitness: 1.0/rs }
    }

    fn evaluate(&mut self) -> f32 { self.fitness }
    fn crossover(&self, _: &Self) -> Self { GATestSolution::new(self.fitness) }
    fn mutate(&mut self, _: f32) {}
    fn fitness(&self) -> f32 { self.fitness }
    fn set_fitness(&mut self, fitness:f32) { self.fitness = fitness; }
    fn score(&self) -> f32 { self.score}
}

pub struct GATestFactory
{
    starting_score: f32
}
impl GATestFactory
{
    pub fn new(starting_score: f32) -> GATestFactory
    {
        GATestFactory {starting_score: starting_score}
    }
}
impl GAFactory<GATestSolution> for GATestFactory
{
    fn initial_population(&mut self) -> GAPopulation<GATestSolution>
    {
        GAPopulation::new(vec![GATestSolution::new(self.starting_score)], GAPopulationSortOrder::HighIsBest)
    }
}
