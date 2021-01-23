import React from "react";
import ProjectsList from "../containers/ProjectsList"
import { useProjects } from "../hooks"

export default function Home() {
    const projects = useProjects();

    return (
        <div className="Content" >
            <h1>Miguel Henriques</h1>
            <p>Hello, my name is Miguel. I'm a Computer Science student ao Iscte in Lisbon</p>
            <ProjectsList projects={projects} maxSize={1} />
        </div>
    )
}