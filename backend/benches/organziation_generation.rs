use advisory_backend_lib::advisories::*;
use advisory_backend_lib::people::*;
use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;

fn create_dummy_teachers(count: u16) -> Result<Vec<Teacher>> {
    let teachers = (0..count)
        .map(|i| Teacher::new(format!("Dummy Teacher {}", i + 1)))
        .collect();

    Ok(teachers)
}

fn create_dummy_teacher_groupings(count: u16) -> Result<Vec<Vec<Teacher>>> {
    let teachers = create_dummy_teachers(count * 2)?;
    let teacher_groupings = teachers.chunks(2).map(|c| c.to_vec()).collect();

    Ok(teacher_groupings)
}

fn create_dummy_students(count: u16) -> Result<Vec<Student>> {
    let mut rng = &mut rand::thread_rng();
    let teachers = create_dummy_teachers(10)?;
    let students = (0..count)
        .map(|_| Student {
            name: "Dummy Student".to_owned(),
            teachers: teachers.choose_multiple(&mut rng, 8).cloned().collect(),
            ..Default::default()
        })
        .collect();

    Ok(students)
}

pub fn generation_speed_benchmark_5(c: &mut Criterion) {
    let settings = Settings {
        weights: Weights::default(),
        num_advisories: 5,
        teacher_groupings: create_dummy_teacher_groupings(5).unwrap(),
    };

    c.bench_function("100 students w/ 5 advisories", |b| {
        b.iter(|| Organization::generate(&settings, create_dummy_students(100).unwrap()))
    });
    c.bench_function("500 students w/ 5 advisories", |b| {
        b.iter(|| Organization::generate(&settings, create_dummy_students(500).unwrap()))
    });
    c.bench_function("1000 students w/ 5 advisories", |b| {
        b.iter(|| Organization::generate(&settings, create_dummy_students(1000).unwrap()))
    });
}

pub fn generation_speed_benchmark_20(c: &mut Criterion) {
    let settings = Settings {
        weights: Weights::default(),
        num_advisories: 20,
        teacher_groupings: create_dummy_teacher_groupings(20).unwrap(),
    };

    c.bench_function("500 students w/ 20 advisories", |b| {
        b.iter(|| Organization::generate(&settings, create_dummy_students(500).unwrap()))
    });
    c.bench_function("2000 students w/ 20 advisories", |b| {
        b.iter(|| Organization::generate(&settings, create_dummy_students(2000).unwrap()))
    });
    c.bench_function("4000 students w/ 20 advisories", |b| {
        b.iter(|| Organization::generate(&settings, create_dummy_students(4000).unwrap()))
    });
}

criterion_group!(
    benches,
    generation_speed_benchmark_5,
    generation_speed_benchmark_20,
);
criterion_main!(benches);
