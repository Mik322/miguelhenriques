import React from "react";
import ProjectsList from "../containers/ProjectsList";
import { Project } from "../types";

interface HomeProps {
  projects: Array<Project>;
}

export default function Home(props: HomeProps) {
  return (
    <div className="Content">
      <h1>Miguel Henriques</h1>
      <p>
        Hello, my name is Miguel. I'm a Computer Science student ao Iscte in
        Lisbon
      </p>
      <ProjectsList projects={props.projects} maxSize={1} />
    </div>
  );
}
