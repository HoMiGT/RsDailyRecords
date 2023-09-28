use pyo3::prelude::*;
use pyo3::types::{PyModule,PyDict};

fn call_python()->PyResult<()>{
    Python::with_gil(|py|{
       let os = py.import("os").unwrap();
        let path:&PyAny = os.getattr("path").unwrap();
        let exists = path.getattr("exists").unwrap();
        let isexists = exists.call1(("/home/c/projects",)).unwrap();
        println!("isexists  D:/Downloads : {:?}",isexists);

        let math_module: &PyModule = py.import("math").unwrap();
        let sqrt = math_module.getattr("sqrt").unwrap();

        let sqrt_res = sqrt.call1((4,)).unwrap();

        println!("path: {:?}",path);

        println!("4 sqrt: {:?}",sqrt_res);

    });
    Ok(())
}

fn call_python1()->PyResult<()>{
    Python::with_gil(|py|{
        let sys = py.import("sys").unwrap();
        let sys_path = sys.getattr("path").unwrap();

        let script_path = "/home/c/projects/tpy/";


        let pydict = PyDict::new(py);

        sys_path.call_method("insert",(0,script_path,),Some(pydict)).unwrap();

        let my_module = py.import("t").unwrap();

        let t = my_module.getattr("t").unwrap();

        let res = t.call1(("/home/c/projects/tpy",)).unwrap();

        println!("script returned:{:?}",res);

    });
    Ok(())
}


fn main() {

    let _ = call_python();
    let _ = call_python1();

    println!("Hello, world!");
}
