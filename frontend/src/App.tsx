import { Route, Routes } from "react-router-dom";
import Main from "./pages/Main";
import ContextMenu from "./pages/ContextMenu";

const App = () => {
    return (
        <div>
            <Routes>
                <Route path="/" element={<Main />} />
                <Route path="/context-menu" element={<ContextMenu />} />
            </Routes>
        </div>
    );
};

export default App;
