import React from "react";
import ProjectsList from "../containers/ProjectsList"
import { useProjects } from "../hooks"

export default function Projects() {
    const projects = useProjects();

    return (
        <div className="Projects" >
            <ProjectsList projects={projects} />
        </div>
    )
}