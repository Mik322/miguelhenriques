import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import { Project, ProjectPath } from "../types";
import { useImageContainer } from "../hooks";

interface ProjectPageProp {
  projectGetter: (id: number) => Project | undefined;
}

export default function ProjectPage(props: ProjectPageProp) {
  let { projectID } = useParams<ProjectPath>();
  let [project, setProject] = useState<Project | null>(null);
  let imageContainer = useImageContainer(project);

  useEffect(() => {
    const id = parseInt(projectID);
    if (!isNaN(id)) {
      const p = props.projectGetter(id);
      setProject(p !== undefined ? p : null);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  if (project === null) return <p>Project does not exist</p>;

  return (
    <div>
      <h2>{project.name}</h2>
      {imageContainer}
      <p>{project.description}</p>
    </div>
  );
}
