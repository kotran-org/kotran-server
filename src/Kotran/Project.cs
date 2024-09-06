using GenHTTP.Api.Content;
using GenHTTP.Modules.Layouting;
using GenHTTP.Modules.Security;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Kotran
{
    public static class Project
    {

        public static IHandlerBuilder Setup()
        {
            return Layout.Create()
                         //.AddService<BookService>("books")
                         .Add(CorsPolicy.Permissive());
        }

    }
}
