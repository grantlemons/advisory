import { type Teacher, type Student, Sex, Grade } from '$lib/DBTypes';
import { read, utils, type WorkSheet, type WorkBook } from 'xlsx';

type Table = string[][];

export function get_teachers(table: Table): Set<Teacher> {
    const teacher_names = new Set<string>();

    let empty_rows = 0;
    for (let index = 0; index < table.length; index += 1) {
        const row = table[index];
        const row_empty = row[8] == undefined;

        if (empty_rows >= 3 && row_empty) break;
        if (row_empty) {
            empty_rows += 1;
        } else {
            empty_rows = 0;
            teacher_names.add(row[8]);
        }
    }
    const teachers = new Set<Teacher>();
    teacher_names.forEach((s) =>
        teachers.add({
            name: s,
        })
    );
    return teachers;
}

export function get_students(table: Table): Set<Student> {
    const students = new Set<Student>();
    let previous_row_empty = false;
    let current_student: Student = {
        name: '',
        teachers: [],
        grade: Grade.Freshman,
        sex: Sex.Male,
        banned_pairings: [],
    };
    const current_student_teachers = new Set<string>();

    for (let index = 0; index < table.length; index += 1) {
        const row = table[index];
        const row_empty: boolean = row[0] == undefined;
        const row_name: string = row[6] + ' ' + row[5];
        const row_grade: Grade = parse_grade_string(row[4]);
        const row_sex: Sex = row[9] as Sex;
        const row_teacher_name: string = row[8];

        if (row_empty && previous_row_empty) {
            break;
        }
        previous_row_empty = row_empty;

        if (row_name != current_student.name) {
            if (current_student.name != '') {
                current_student_teachers.forEach((s) =>
                    current_student.teachers.push({ name: s })
                );
                console.log(current_student);
                students.add(current_student);
                current_student_teachers.clear();
            }
            current_student = {
                name: row_name,
                teachers: [],
                grade: row_grade,
                sex: row_sex,
                banned_pairings: [],
            };
        }
        if (row_teacher_name != undefined) {
            current_student_teachers.add(row_teacher_name);
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

export function import_table(buffer: ArrayBuffer): Table {
    const workbook: WorkBook = read(buffer);

    const table: Table = sheet_to_aoa(workbook.Sheets['Schedules']);
    return table.slice(1);
}

function sheet_to_aoa(sheet: WorkSheet): Table {
    return utils.sheet_to_json(sheet, { header: 1 });
}

export function sets_from_table(
    buffer: ArrayBuffer
): [Set<Teacher>, Set<Student>] {
    const table = import_table(buffer);

    const teachers: Set<Teacher> = get_teachers(table);
    const students: Set<Student> = get_students(table);

    return [teachers, students];
}
