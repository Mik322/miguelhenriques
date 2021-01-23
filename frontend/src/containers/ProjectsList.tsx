import React from "react";
import {ProjectList} from "../types";
import ProjectCard from "../components/ProjectCard";
import "../static/ProjectsList.css"

export default function ProjectsList({projects, maxSize=0}: ProjectList) {
    const projectCards: Array<any> = [];
    let ProjectList = projects;

    if (maxSize>0) ProjectList = projects.slice(0, maxSize)

    ProjectList.forEach(project => {
        projectCards.push(<ProjectCard key={project.id} project={project} />)
    })
    return(
        <div className="projectsList" >
            {projectCards}
        </div>
    )
}