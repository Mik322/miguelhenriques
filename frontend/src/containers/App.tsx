import React, { useState, useEffect } from "react";
import "../static/App.css";
import Home from "../pages/Home";
import Navbar from "./Navbar";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import Projects from "../pages/Projects";
import ContactMe from "../pages/ContactMe";
import ProjectPage from "../pages/ProjectPage";
import Admin from "../admin/Admin";
import { Project } from "../types";
import { getProjects } from "../api";

function App() {
  const [projects, setProjects] = useState<Array<Project>>([]);

  const getProjectById = (id: number): Project | undefined => {
    return projects.find((p) => p.id === id);
  };

  useEffect(() => {
    getProjects().then((prjs) => setProjects(prjs));
  }, []);

  return (
    <Router>
      <div className="App">
        <Switch>
          <Route path="/admin">
            <Admin />
          </Route>
          <Route path="/">
            <Navbar />
            <Switch>
              <Route path="/projects">
                <Projects projects={projects} />
              </Route>

              <Route path="/contact-me">
                <ContactMe />
              </Route>

              <Route path="/project/:projectID">
                <ProjectPage projectGetter={getProjectById} />
              </Route>
              <Route path="/">
                <Home projects={projects} />
              </Route>
            </Switch>
          </Route>
        </Switch>
      </div>
    </Router>
  );
}

export default App;
