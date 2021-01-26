import React from "react";
import ProjectsList from "../containers/ProjectsList";
import { Project } from "../types";

interface ProjectsProps {
  projects: Array<Project>;
}

export default function Projects(props: ProjectsProps) {
  return (
    <div className="Projects">
      <ProjectsList projects={props.projects} />
    </div>
  );
}
