import {useEffect, useState} from "react";
import { Project } from "./types";
import { getProjects } from "./api";

export function useProjects() {
    const [projects, setProjects] = useState<Array<Project>>([]);

    useEffect(() => {
        setProjects(getProjects())
    }, [])

    return projects
}