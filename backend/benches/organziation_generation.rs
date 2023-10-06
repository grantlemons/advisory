use advisory_backend_lib::advisories::*;
use advisory_backend_lib::people::*;
use anyhow::Result;
use criterion::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use std::sync::Arc;

fn create_dummy_teachers(count: u16) -> Result<Arc<[Teacher]>> {
    let teachers = (0..count)
        .map(|i| Teacher::new(format!("Dummy Teacher {}", i + 1)))
        .collect();

    Ok(teachers)
}

fn create_dummy_teacher_groupings(count: u16) -> Result<Arc<[Arc<[Teacher]>]>> {
    let teachers = create_dummy_teachers(count * 2)?;
    let teacher_groupings = teachers
        .chunks(2)
        .map(|c| c.to_vec().into())
        .collect::<Arc<[Arc<[_]>]>>();

    Ok(teacher_groupings)
}

fn create_dummy_students(count: u16) -> Result<Arc<[Student]>> {
    let mut rng = &mut rand::thread_rng();
    let teachers = create_dummy_teachers(10)?;
    let students = (0..count)
        .map(|_| Student {
            name: Arc::new("Dummy Student".to_owned()),
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

    for student_count in [100, 500, 1000] {
        let title = format!("{} students w/ 5 advisories", student_count);
        c.bench_function(&title, |b| {
            b.iter(|| {
                Organization::generate(
                    &settings,
                    black_box(create_dummy_students(student_count).unwrap()),
                )
            })
        });
    }
}

pub fn generation_speed_benchmark_20(c: &mut Criterion) {
    let settings = Settings {
        weights: Weights::default(),
        num_advisories: 20,
        teacher_groupings: create_dummy_teacher_groupings(20).unwrap(),
    };

    for student_count in [100, 500, 1000] {
        let title = format!("{} students w/ 20 advisories", student_count);
        c.bench_function(&title, |b| {
            b.iter(|| {
                Organization::generate(
                    &settings,
                    black_box(create_dummy_students(student_count).unwrap()),
                )
            })
        });
    }
}

criterion_group!(
    benches,
    generation_speed_benchmark_5,
    generation_speed_benchmark_20,
);
criterion_main!(benches);
