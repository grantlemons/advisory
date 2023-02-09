import { type Teacher, type Student, Sex, Grade } from '$lib/DBTypes';

type Table = string[][];

export function get_teachers(table: Table): Set<Teacher> {
    const teachers = new Set<Teacher>();
    for (const row in table) {
        teachers.add({
            name: row[8],
        });
    }
    return teachers;
}

export function get_students(table: Table): Set<Student> {
    const students = new Set<Student>();
    let current_student: Student = {
        name: '',
        teachers: [],
        grade: Grade.Freshman,
        sex: Sex.Male,
    };

    for (const row in table) {
        const row_name: string = row[6] + row[7];
        const row_grade: Grade = parse_grade_string(row[4]);
        const row_sex: Sex = row[9] as Sex;
        const row_teacher_name: string = row[8];

        if (row_name != current_student.name) {
            students.add(current_student);
            current_student = {
                name: row_name,
                teachers: [],
                grade: row_grade,
                sex: row_sex,
            };
        }

        if (row_teacher_name.length != 0) {
            current_student.teachers.push({ name: row_teacher_name });
        }
    }
    return students;
}

function parse_grade_string(grade: string): Grade {
    let value: Grade = Grade.Freshman;
    switch (parseInt(grade)) {
        case 9: {
            value = Grade.Freshman;
            break;
        }
        case 10: {
            value = Grade.Sophomore;
            break;
        }
        case 11: {
            value = Grade.Junior;
            break;
        }
        case 12: {
            value = Grade.Senior;
            break;
        }
    }
    return value;
}
