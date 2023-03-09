export enum Sex {
    Male = 'Male',
    Female = 'Female',
}
export enum Grade {
    Freshman = 'Freshman',
    Sophomore = 'Sophomore',
    Junior = 'Junior',
    Senior = 'Senior',
}
export interface Teacher {
    name: string;
}
export interface Person {
    name: string;
}
export interface Student {
    name: string;
    sex: Sex;
    teachers: Teacher[];
    grade: Grade;
    banned_pairings: [];
}

export interface Advisory {
    // user_id: string;
    advisors: Teacher[];
    students: Student[];
    // remaining_sex: [number, number];
    // remaining_grade: [number, number, number, number];
}

export interface Weights {
    has_teacher: number;
    sex_diverse: number;
    grade_diverse: number;
}

export interface Settings {
    weights: Weights;
    num_advisories: number;
    teacher_groupings: Teacher[][];
}
