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
    sex: Sex;
}
export interface Student {
    name: string;
    sex: Sex;
    teachers: Teacher[];
    grade: Grade;
}
