import { Route, Routes } from "react-router-dom";
import Main from "./pages/Main";
import ContextMenu from "./pages/ContextMenu";
import ContextSubMenu from "./pages/ContextSubMenu";

const App = () => {
    return (
        <div>
            <Routes>
                <Route path="/" element={<Main />} />
                <Route path="/context-menu" element={<ContextMenu />}>
                    <Route path="submenu" element={<ContextSubMenu />} />
                </Route>
            </Routes>
        </div>
    );
};

export default App;
