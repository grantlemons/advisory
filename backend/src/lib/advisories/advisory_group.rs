use crate::advisories::Advisory;
use crate::people::Teacher;

pub(crate) struct AdvisoryGroup(pub(crate) Vec<Advisory>);

impl AdvisoryGroup {
    pub(crate) fn default(student_count: i16, advisory_count: i16) -> Self {
        let students_per_advisory = student_count / advisory_count;

        Self(vec![
            Advisory::default(students_per_advisory);
            advisory_count.try_into().unwrap()
        ])
    }

    pub(crate) fn populate_teachers(&mut self, teacher_pairs: &[[Option<Teacher>; 2]]) {
        for (index, target_advisory) in self.0.iter_mut().enumerate() {
            let [t1, t2] = teacher_pairs[index].clone();

            log::info!("Adding {:?} to {}", vec![&t1, &t2], target_advisory);
            target_advisory.add_teacher(t1);
            target_advisory.add_teacher(t2);
        }
    }
}
