import React from "react";
import "../static/App.css";
import Home from "../pages/Home";
import Navbar from "./Navbar";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import Projects from "../pages/Projects";
import ContactMe from "../pages/ContactMe";
import ProjectPage from "../pages/ProjectPage";
import Admin from "../admin/Admin";

function App() {
  return (
    <Router>
      <div className="App">
        <Switch>
          <Route path="/admin">
            <Admin />
          </Route>

          <Route path="/projects">
            <Navbar />
            <Projects />
          </Route>

          <Route path="/contact-me">
            <Navbar />
            <ContactMe />
          </Route>

          <Route path="/project/:projectID">
            <Navbar />
            <ProjectPage />
          </Route>

          <Route path="/">
            <Navbar />
            <Home />
          </Route>
        </Switch>
      </div>
    </Router>
  );
}

export default App;
