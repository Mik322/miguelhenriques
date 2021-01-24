export type Project = {
  id: number;
  image_name: String | null;
  name: String;
  description: String;
};

export type ProjectProp = {
  project: Project;
};

export interface ProjectList {
  projects: Array<Project>;
  maxSize?: number;
}

export type Email = {
  name: String;
  from: String;
  subject: String;
  text: String;
};

export interface ProjectPath {
  projectID: string;
}
