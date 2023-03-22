/* eslint-disable @typescript-eslint/no-explicit-any */
import axios, { type AxiosResponse } from 'axios';
import { id_token } from '$lib/auth_store';
import type {
    Teacher,
    Advisory,
    Student,
    Weights,
    Settings,
    Person,
} from '$lib/DBTypes';

let auth: string;
id_token.subscribe((value) => {
    auth = value;
});

export default class API {
    static BASE_URL = '/api';

    static clean_database(): Promise<AxiosResponse<any, any>> {
        return axios.delete(`${this.BASE_URL}/people`, {
            headers: {
                Authorization: auth,
            },
        });
    }

    static add_teacher(teacher: Teacher): Promise<AxiosResponse<any, any>> {
        return axios({
            method: 'post',
            url: `${this.BASE_URL}/people/teacher`,
            data: teacher,
            headers: {
                Authorization: auth,
            },
        });
    }

    static add_teachers_bulk(
        teachers: Teacher[]
    ): Promise<AxiosResponse<any, any>> {
        return axios({
            method: 'post',
            url: `${this.BASE_URL}/people/teacher/bulk`,
            data: teachers,
            headers: {
                Authorization: auth,
            },
        });
    }

    static add_student(student: Student): Promise<AxiosResponse<any, any>> {
        return axios({
            method: 'post',
            url: `${this.BASE_URL}/people/student`,
            data: student,
            headers: {
                Authorization: auth,
            },
        });
    }

    static add_students_bulk(
        students: Student[]
    ): Promise<AxiosResponse<any, any>> {
        return axios({
            method: 'post',
            url: `${this.BASE_URL}/people/student/bulk`,
            data: students,
            headers: {
                Authorization: auth,
            },
        });
    }

    static ban_pairing(
        p1: Person,
        p2: Person
    ): Promise<AxiosResponse<any, any>> {
        return axios({
            method: 'post',
            url: `${this.BASE_URL}/people/ban`,
            data: [p1, p2],
            headers: {
                Authorization: auth,
            },
        });
    }

    static list_people(): Promise<AxiosResponse<any, any>> {
        return axios<Person[]>({
            method: 'get',
            url: `${this.BASE_URL}/people`,
            headers: {
                Authorization: auth,
            },
        });
    }

    static list_teachers(): Promise<AxiosResponse<any, any>> {
        return axios<Teacher[]>({
            method: 'get',
            url: `${this.BASE_URL}/people/teacher`,
            headers: {
                Authorization: auth,
            },
        });
    }

    static list_students(): Promise<AxiosResponse<any, any>> {
        return axios<Student[]>({
            method: 'get',
            url: `${this.BASE_URL}/people/student`,
            headers: {
                Authorization: auth,
            },
        });
    }

    static get_advisories(
        teacher_groupings: Teacher[][],
        weights: Weights
    ): Promise<AxiosResponse<Advisory[], any>> {
        const data: Settings = {
            weights,
            num_advisories: teacher_groupings.length,
            teacher_groupings,
        };

        return axios<Advisory[]>({
            method: 'put',
            url: `${this.BASE_URL}/`,
            data,
            headers: {
                Authorization: auth,
            },
        });
    }
}
