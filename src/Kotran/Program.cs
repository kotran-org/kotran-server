using GenHTTP.Engine;
using GenHTTP.Modules.Practices;

namespace Kotran
{
    internal class Program
    {
        static int Main(string[] args)
        {
            var project = Project.Setup();

            return Host.Create()
                       .Handler(project)
                       .Defaults()
                       .Console()
            //-:cnd:noEmit
            #if DEBUG
                       .Development()
            #endif
            //+:cnd:noEmit
                       .Run();
        }
    }
}
