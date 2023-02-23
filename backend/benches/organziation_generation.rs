use advisory_backend_lib::advisories::*;
use advisory_backend_lib::people::*;
use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;

fn create_dummy_teachers(count: u16) -> Result<Vec<Teacher>> {
    let mut teachers = Vec::new();

    for index in 0..count {
        teachers.push(Teacher::new(format!("Dummy Teacher {}", index + 1)));
    }

    Ok(teachers)
}

fn create_dummy_teacher_pairs(count: u16) -> Result<Vec<[Option<Teacher>; 2]>> {
    let mut teachers = create_dummy_teachers(count * 2)?;
    let mut teacher_pairs: Vec<[Option<Teacher>; 2]> = Vec::new();

    for _ in 0..count {
        teacher_pairs.push([teachers.pop(), teachers.pop()]);
    }
    Ok(teacher_pairs)
}

fn create_dummy_students(count: u16) -> Result<Vec<Student>> {
    let mut rng = &mut rand::thread_rng();
    let teachers: Vec<Teacher> = create_dummy_teachers(10)?;
    let mut students = Vec::new();

    for _ in 0..count {
        students.push(Student {
            name: "Dummy Student".to_owned(),
            teachers: teachers
                .choose_multiple(&mut rng, 8)
                .cloned()
                .collect::<Vec<_>>(),
            ..Default::default()
        });
    }
    Ok(students)
}

pub fn generation_speed_benchmark_5(c: &mut Criterion) {
    let settings: Settings = Settings {
        weights: Weights {
            has_teacher: 1,
            sex_diverse: 1,
            grade_diverse: 1,
        },
        num_advisories: 5,
        teacher_pairs: create_dummy_teacher_pairs(5).unwrap(),
    };

    c.bench_function("generate 100 students w/ 5 advisories", |b| {
        b.iter(|| Organization::generate(settings.clone(), create_dummy_students(100).unwrap()))
    });
    c.bench_function("generate 500 students w/ 5 advisories", |b| {
        b.iter(|| Organization::generate(settings.clone(), create_dummy_students(500).unwrap()))
    });
    c.bench_function("generate 1000 students w/ 5 advisories", |b| {
        b.iter(|| Organization::generate(settings.clone(), create_dummy_students(1000).unwrap()))
    });
}

pub fn generation_speed_benchmark_20(c: &mut Criterion) {
    let settings: Settings = Settings {
        weights: Weights {
            has_teacher: 1,
            sex_diverse: 1,
            grade_diverse: 1,
        },
        num_advisories: 20,
        teacher_pairs: create_dummy_teacher_pairs(20).unwrap(),
    };

    c.bench_function("generate 500 students w/ 20 advisories", |b| {
        b.iter(|| Organization::generate(settings.clone(), create_dummy_students(500).unwrap()))
    });
    c.bench_function("generate 2000 students w/ 20 advisories", |b| {
        b.iter(|| Organization::generate(settings.clone(), create_dummy_students(2000).unwrap()))
    });
    c.bench_function("generate 4000 students w/ 20 advisories", |b| {
        b.iter(|| Organization::generate(settings.clone(), create_dummy_students(4000).unwrap()))
    });
}

criterion_group!(
    benches,
    generation_speed_benchmark_5,
    generation_speed_benchmark_20,
);
criterion_main!(benches);
