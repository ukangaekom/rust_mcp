use std::sync::{Arc, Mutex};



fn main() {
    


}


#[derived(Clone)]
pub struct HelloWorld{
    counter: Arc<Mutex<i32>>,

}


impl HelloWorld{

    pub fn new() -> Self{

        Self{
            counter: Arc::new(Mutex::new(0));
        }


    }



    #[tool(description= "Increments the counter by 1")]
    async fn incremental(&self) -> Result<CallToolResult, Error>{
        let mut counter = self.counter.lock().await;

        *counter += 1;

        Ok(
            CallToolResult::success(vec![
                Content::text(
                    counter.to_string(),
                )
            ])
        )

    }


    
    #[tool(description= "Decrements the counter by 1")]
    async fn decrement(&self) -> Result<CallToolResult, Error>{
        let mut counter = self.counter.lock().await;

        *counter -= 1;

        Ok(
            CallToolResult::success(vec![
                Content::text(
                    counter.to_string(),
                )
            ])
        )

    }


    // Creating a getting tool
    #[tool(description="Get the current counter value")]
    async fn get_value(&self) -> Result<CallToolResult, Error>

        let mut counter = self.counter.lock().await;

        Ok(

            CallToolResult::success(
                vec![Content::text(
                    counter.to_string(),
            )]
            )
        )


    }

    // Creating an echo tool
    #[tool(description="Say something")]
    pub fn echo(&self) -> Result<CallToolResult, Error>{

        Ok(
            CallToolResult::success(
                vec![Content::text::String::from("Hello from your first MCP Server")]
            )
        )

    }

}



impl ServerHandler for HelloWorld{
    
}
