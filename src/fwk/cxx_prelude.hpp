/*
 * niepce - fwk/cxx_prelude.hpp
 */

#pragma once

#include <memory>

// things that need to be declared before anything.
// early "extern C++"
// And that the implementation needs too.
namespace fwk {
class Application;
class Moniker;
class PropertyValue;
class WrappedPropertyBag;
class SharedConfiguration;
typedef std::shared_ptr<SharedConfiguration> ConfigurationPtr;
}

namespace eng {
class Label;
class LcChannel;
class LibNotification;
class ThumbnailCache;
}

typedef struct _GdkPixbuf GdkPixbuf;
typedef struct _GtkWidget GtkWidget;

namespace npc {
class LnListener;
}
